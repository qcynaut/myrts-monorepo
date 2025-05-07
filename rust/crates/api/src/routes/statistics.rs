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
use actix_web::{web, Result};
use api_db::{
    repos::{AvsRepo, RecordsRepo, ScheduleRepo, UserGroupRepo},
    ApiDatabase,
};
use types::api::{user::User, StatisticsRes};

/// # Get user statistics.
///
/// This endpoint responsible for getting user statistics.
/// ---
/// tags:
///   - statistics
/// responses:
///     - status: 200
///       content: !T StatisticsRes
///       description: The user statistics.
///     - status: 401
///       content: !T ApiError
///       description: Unauthorized.
///     - status: 500
///       content: !T ApiError
///       description: Server encountered an error.
/// auth: api_key
#[api_rt::route(get, "/statistics", Auth)]
async fn get(user: web::ReqData<User>, db: web::Data<ApiDatabase>) -> Result<StatisticsRes> {
    let user_id = user.id;
    let group_repo = db.repository::<UserGroupRepo>();
    let records_repo = db.repository::<RecordsRepo>();
    let schedule_repo = db.repository::<ScheduleRepo>();
    let avs_repo = db.repository::<AvsRepo>();

    let unit = group_repo.count(user_id).map_err(ApiError::from)?;
    let records = records_repo.get_all(user_id).map_err(ApiError::from)?;
    let schedule = if user.role_id == 3 {
        schedule_repo.get_by_user(user_id).map_err(ApiError::from)?
    } else {
        schedule_repo.get_all().map_err(ApiError::from)?
    };

    let mut duration = 0;

    for record in &records {
        duration += record.duration.parse().unwrap_or(0);
    }

    let avs = avs_repo.get_all().map_err(ApiError::from)?;
    let avs_len = if user.role_id == 3 {
        user.device_ids.len()
    } else {
        avs.len()
    };

    StatisticsRes::new(
        duration,
        schedule.len(),
        records.len(),
        unit as usize,
        avs_len as usize,
    )
    .wrap()
}

api_rt::routes! {
    get
}
