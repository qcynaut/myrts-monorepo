/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use crate::{middlewares::auth::Auth, ApiError};
use actix_web::web;
use api_db::{
    repos::{AvsRepo, RecordsRepo, ScheduleRepo},
    ApiDatabase,
};
use proto::WsState;
use timeslots::TimeSlots;
use types::api::{
    schedules::{NewSchedule, NewScheduleReq, SchedulesResponse},
    user::User,
};

/// # Get all schedules.
///
/// This endpoint responsible for getting all schedules. If the user role `1 (Root)` or `2 (SuperAdmin)` it will return all schedules
/// Otherwise it will return only schedules created by the user.
/// ---
/// tags:
///     - schedules
/// responses:
///     - status: 200
///       content: !Vec SchedulesResponse
///       content_type: application/json
///       description: Created schedule.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/schedules", Auth)]
async fn get(
    user: web::ReqData<User>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<ScheduleRepo>();
    if user.role_id == 1 || user.role_id == 2 {
        let res = repo.get_all().map_err(ApiError::from)?;
        SchedulesResponse::wrap_vec(res.into_iter().map(SchedulesResponse::from).collect(), None)
    } else {
        let res = repo.get_by_user(user.id).map_err(ApiError::from)?;
        SchedulesResponse::wrap_vec(res.into_iter().map(SchedulesResponse::from).collect(), None)
    }
}

/// # Get schedule by id.
///
/// This endpoint responsible for getting a schedule by id.
/// ---
/// tags:
///     - schedules
/// responses:
///     - status: 200
///       content: !T SchedulesResponse
///       content_type: application/json
///       description: Created schedule.
///     - status: 400
///       content: !T ApiError
///       content_type: application/json
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Unauthorized.
///     - status: 404
///       content: !T ApiError
///       content_type: application/json
///       description: Not found.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
///       description: The id of the schedule.
/// auth: api_key
#[api_rt::route(get, "/schedules/{id}", Auth)]
async fn get_by_id(
    user: web::ReqData<User>,
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<SchedulesResponse> {
    let repo = db.repository::<ScheduleRepo>();
    let res = repo.get(id.into_inner()).map_err(ApiError::from)?;
    if user.role_id == 1 || user.role_id == 2 {
        SchedulesResponse::from(res).wrap()
    } else {
        if res.user_id != user.id {
            return Err(ApiError::new("Not found").status(404).into());
        }
        SchedulesResponse::from(res).wrap()
    }
}

/// # Get schedule by the given avs id.
///
/// This endpoint responsible for getting a schedule by the given avs id,
/// If the user role `1 (Root)` or `2 (SuperAdmin)` it will return all schedules
/// on the device, otherwise it will return schedule on the device if user have control to the device.
/// ---
/// tags:
///     - schedules
/// responses:
///     - status: 200
///       content: !Vec SchedulesResponse
///       content_type: application/json
///       description: Created schedule.
///     - status: 400
///       content: !T ApiError
///       content_type: application/json
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Unauthorized.
///     - status: 404
///       content: !T ApiError
///       content_type: application/json
///       description: Not found.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// params:
///     - name: avs
///       kind: !Path i32
///       required: true
///       description: Avs id.
/// auth: api_key
#[api_rt::route(get, "/schedules/avs/{avs}", Auth)]
async fn get_by_avs(
    user: web::ReqData<User>,
    avs: web::Path<i32>,
    db: web::Data<ApiDatabase>,
) -> actix_web::Result<impl actix_web::Responder> {
    let repo = db.repository::<ScheduleRepo>();
    if user.role_id == 1 || user.role_id == 2 {
        let res = repo.get_by_avs(avs.into_inner()).map_err(ApiError::from)?;
        SchedulesResponse::wrap_vec(res.into_iter().map(SchedulesResponse::from).collect(), None)
    } else {
        let id = avs.into_inner();
        let user_avs_ids: Vec<Option<i32>> = user
            .device_ids
            .clone()
            .into_iter()
            .filter(|x| x.is_some())
            .collect();
        if user_avs_ids.contains(&Some(id)) {
            let res = repo.get_by_avs(id).map_err(ApiError::from)?;
            SchedulesResponse::wrap_vec(
                res.into_iter().map(SchedulesResponse::from).collect(),
                None,
            )
        } else {
            Err(ApiError::new("Not found").status(404).into())
        }
    }
}

/// # Create a new schedule.
///
/// This endpoint responsible for creating a new schedule.
///
/// The volumes field in the request body indicating the volume of the schedule, if empty it will be using default volume.
///
/// If you want to set the custom volume for device with id 1 = 50%, and 3 = 30%, then the request body must be like this:
/// `volumes`: ["1:50", "3:30"]
///
/// ****Rules:****
/// - The record must be in active `1 (Active)` status.
/// - The record used must be owned by the user.
/// - The fields used must be match with it's kind `1 (Repetition)` or `2 (Calendar)`.
///
/// ****Example:****
///
/// If you want to create a new repetition schedule with the following conditions:
///
/// Create a schedule that will repeated every first week of the month at sunday, thursday, and friday every 10:30, 15:00, and 19:00.
/// The request body must be like this:
///
/// - `name`: My Schedule,
/// - `kind`: 1,
/// - `weeks`: [1],
/// - `days`: [1, 4, 5],
/// - `times`: [10:30, 15:00, 19:00],
/// - `month`: null,
/// - `year`: null,
/// - `dates`: [],
/// - `records_id`: 1
/// - `device_ids`: [],
///
/// If you want to create a new calendar schedule with the following conditions:
///
/// Create a schedule that will play on August 2024 on 1, 8, 15, 22, and 29 day of the month at 10:30, 15:00, 19:00.
/// The request body must be like this:
///
/// - `name`: My Schedule,
/// - `kind`: 2,
/// - `weeks`: [],
/// - `days`: [],
/// - `times`: [10:30, 15:00, 19:00],
/// - `dates`: [1, 8, 15, 22, 29],
/// - `month`: 8,
/// - `year`: 2024,
/// - `records_id`: 1
/// - `device_ids`: [],
///
/// ---
/// tags:
///     - schedules
/// request:
///     content: !T NewScheduleReq
///     content_type: application/json
///     description: New schedule.
/// responses:
///     - status: 200
///       content: !T SchedulesResponse
///       content_type: application/json
///       description: Created schedule.
///     - status: 400
///       content: !T ApiError
///       content_type: application/json
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 404
///       content: !T ApiError
///       content_type: application/json
///       description: Not found.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(post, "/schedules", Auth)]
async fn post(
    user: web::ReqData<User>,
    data: web::Json<NewScheduleReq>,
    db: web::Data<ApiDatabase>,
    ws_state: web::Data<WsState>,
) -> actix_web::Result<SchedulesResponse> {
    if data.device_ids.is_empty() {
        return Err(ApiError::new("Device ids cannot be empty.")
            .status(400)
            .into());
    }

    // Check records.
    let record_repo = db.repository::<RecordsRepo>();
    let record = record_repo.get(data.records_id).map_err(ApiError::from)?;
    if record.user_id != user.id || record.status != 1 {
        return Err(ApiError::new("Record not found.").status(404).into());
    }

    // Check avs.
    let avs_repo = db.repository::<AvsRepo>();
    let avs_ids = data
        .device_ids
        .iter()
        .filter(|id| id.is_some())
        .map(|id| id.unwrap())
        .collect::<Vec<i32>>();
    let avses = avs_repo.get_many(&avs_ids).map_err(ApiError::from)?;

    if avses.is_empty() || avses.len() != avs_ids.len() {
        return Err(ApiError::new("Avs not found.").status(404).into());
    }

    // Check schedule.
    let schedule_repo = db.repository::<ScheduleRepo>();
    let data = data.into_inner();

    let mut avs_ids = Vec::new();
    let duration_secs = record.duration.parse::<u64>().unwrap();
    let mut duration = duration_secs / 60;
    if duration_secs % 60 != 0 {
        duration += 1;
    }
    for avs in avses {
        avs_ids.push(avs.unique_id.clone());
        if avs.pending != 0 {
            return Err(ApiError::new("Avs not found.").status(404).into());
        }

        let mut slots = TimeSlots::try_from(avs.slots.clone()).map_err(|e| ApiError::new(e))?;

        // check for repetition
        if data.times.is_empty() {
            return Err(ApiError::new("Times cannot be empty.").status(400).into());
        }
        let times = data
            .times
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<String>>();
        let times = times
            .into_iter()
            .map(|t| {
                let mut parts = t.split(':');
                let hour = parts.next().unwrap().parse::<u8>().unwrap();
                let minute = parts.next().unwrap().parse::<u8>().unwrap();
                (hour, minute)
            })
            .collect::<Vec<(u8, u8)>>();
        if data.kind == 1 {
            if !data.weeks.is_empty() {
                if data.days.is_empty() {
                    return Err(ApiError::new("Days cannot be empty.").status(400).into());
                }
                // let weeks = data
                //     .weeks
                //     .clone()
                //     .into_iter()
                //     .flatten()
                //     .collect::<Vec<i32>>();
                // let days = data
                //     .days
                //     .clone()
                //     .into_iter()
                //     .flatten()
                //     .collect::<Vec<i32>>();
                // for week in weeks {
                //     for day in days.clone() {
                //         for (hour, minute) in times.clone() {
                //             let filled = slots.add_week(
                //                 week as u8,
                //                 day as u8,
                //                 hour as u8,
                //                 minute as u8,
                //                 duration as u32,
                //             );
                //             if filled {
                //                 return Err(ApiError::new(format!("Schedule already filled. week: {week}, day: {day}, time: {hour}:{minute}")).status(400).into());
                //             }
                //         }
                //     }
                // }
            } else if !data.dates.is_empty() {
                // let dates = data
                //     .dates
                //     .clone()
                //     .into_iter()
                //     .flatten()
                //     .collect::<Vec<i32>>();
                // for date in dates {
                //     for (hour, minute) in times.clone() {
                //         let filled =
                //             slots.add(date as u8, hour as u8, minute as u8, duration as u32);
                //         if filled {
                //             return Err(ApiError::new(format!(
                //                 "Schedule already filled. date: {date}, time: {hour}:{minute}"
                //             ))
                //             .status(400)
                //             .into());
                //         }
                //     }
                // }
            } else {
                return Err(ApiError::new("Weeks or dates cannot be empty.")
                    .status(400)
                    .into());
            }
        } else if data.kind == 2 {
            if data.year.is_none() || data.month.is_none() || data.dates.is_empty() {
                return Err(ApiError::new("Year, month, and dates cannot be empty.")
                    .status(400)
                    .into());
            }
            // let year = data.year.clone().unwrap();
            // let month = data.month.clone().unwrap();
            // let dates = data
            //     .dates
            //     .clone()
            //     .into_iter()
            //     .flatten()
            //     .collect::<Vec<i32>>();
            // for date in dates {
            //     for (hour, minute) in times.clone() {
            //         let filled = slots.add_once(
            //             year as u16,
            //             month as u8,
            //             date as u8,
            //             hour,
            //             minute,
            //             duration as u32,
            //         );
            //         if filled {
            //             return Err(ApiError::new(format!(
            //                 "Schedule already filled. date: {date}, time: {hour}:{minute}"
            //             ))
            //             .status(400)
            //             .into());
            //         }
            //     }
            // }
        } else {
            return Err(ApiError::new("Kind must be 1 or 2.").status(400).into());
        }
        // let slots: String = slots.into();
        // let _ = avs_repo.update_slots(avs.id, &slots);
    }

    let res = schedule_repo
        .create(NewSchedule {
            name: data.name,
            days: data.days,
            records_id: data.records_id,
            device_ids: data.device_ids,
            kind: data.kind,
            weeks: data.weeks,
            dates: data.dates,
            times: data.times,
            user_id: user.id,
            month: data.month,
            year: data.year,
            volumes: data.volumes,
        })
        .map_err(ApiError::from)?;
    tokio::spawn(async move {
        for id in avs_ids {
            if let Some(av) = ws_state.avs_by_id(id).await {
                let _ = av.write("resync", "").await;
            }
        }
    });
    SchedulesResponse::from(res).wrap()
}

/// # Delete schedule.
///
/// This endpoint responsible for deleting a schedule.
/// Root and SuperAdmin can delete any schedule, while Admin can only delete their own schedule.
/// ---
/// tags:
///     - schedules
/// responses:
///     - status: 200
///       content: !T SchedulesResponse
///       content_type: application/json
///       description: Deleted schedule.
///     - status: 400
///       content: !T ApiError
///       content_type: application/json
///       description: Bad request.
///     - status: 401
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
///     - status: 404
///       content: !T ApiError
///       content_type: application/json
///       description: Not found.
///     - status: 500
///       content: !T ApiError
///       content_type: application/json
///       description: Server encountered an error.
/// params:
///     - name: id
///       kind: !Path i32
///       required: true
///       description: Schedule id
/// auth: api_key
#[api_rt::route(delete, "/schedules/{id}", Auth)]
async fn delete(
    user: web::ReqData<User>,
    id: web::Path<i32>,
    db: web::Data<ApiDatabase>,
    ws_state: web::Data<WsState>,
) -> actix_web::Result<SchedulesResponse> {
    let repo = db.repository::<ScheduleRepo>();
    let avs_repo = db.repository::<AvsRepo>();
    if user.role_id == 1 || user.role_id == 2 {
        let res = repo.delete(id.into_inner()).map_err(ApiError::from)?;
        let avs_ids_c = res.device_ids.clone();
        let mut avs_ids = Vec::new();
        for id in avs_ids_c {
            if let Some(id) = id {
                avs_ids.push(id);
            }
        }
        let avses = avs_repo.get_many(&avs_ids).map_err(ApiError::from)?;
        let avs_ids = avses
            .into_iter()
            .map(|avs| avs.unique_id)
            .collect::<Vec<String>>();
        tokio::spawn(async move {
            for id in avs_ids {
                if let Some(av) = ws_state.avs_by_id(id).await {
                    let _ = av.write("resync", "").await;
                }
            }
        });

        SchedulesResponse::from(res).wrap()
    } else {
        let id = id.into_inner();
        let current = repo.get(id).map_err(ApiError::from)?;
        if current.user_id != user.id {
            return Err(ApiError::new("Not found.").status(404).into());
        }
        let res = repo.delete(id).map_err(ApiError::from)?;
        let avs_ids_c = res.device_ids.clone();
        let mut avs_ids = Vec::new();
        for id in avs_ids_c {
            if let Some(id) = id {
                avs_ids.push(id);
            }
        }
        let avses = avs_repo.get_many(&avs_ids).map_err(ApiError::from)?;
        let avs_ids = avses
            .into_iter()
            .map(|avs| avs.unique_id)
            .collect::<Vec<String>>();
        tokio::spawn(async move {
            for id in avs_ids {
                if let Some(av) = ws_state.avs_by_id(id).await {
                    let _ = av.write("resync", "").await;
                }
            }
        });
        SchedulesResponse::from(res).wrap()
    }
}

api_rt::routes! {
    get
    get_by_id
    get_by_avs
    post
    delete
}
