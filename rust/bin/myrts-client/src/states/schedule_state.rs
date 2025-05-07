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

use audio::audio::AudioPlayer;
use proto_db::{repos::ScheduleRepo, ProtoDatabase};
use rodio::Decoder;
use std::{fs::File, io::BufReader, sync::Arc};
use tokio::{sync::RwLock, task::JoinHandle};
use types::proto::Schedule;
use utils::{
    files::ApiAssets,
    time::{current_month, current_year, day_no, day_of_week, week_no},
};

/// Schedule state holds the state of the scheduler.
#[derive(Clone)]
pub struct ScheduleState {
    assets: ApiAssets,
    runner: Arc<RwLock<Vec<JoinHandle<()>>>>,
    block: Arc<RwLock<bool>>,
    db: ProtoDatabase,
    player: Arc<AudioPlayer>,
}

impl ScheduleState {
    /// New schedule state.
    pub fn new(db: ProtoDatabase, assets: ApiAssets, player: Arc<AudioPlayer>) -> Self {
        Self {
            assets,
            runner: Arc::new(RwLock::new(vec![])),
            block: Arc::new(RwLock::new(false)),
            db,
            player,
        }
    }

    /// Clear runner.
    async fn clear_runner(&self) {
        let mut lock = self.runner.write().await;
        for runner in lock.drain(..) {
            runner.abort();
        }
    }

    /// Download record if it doesn't exist.
    async fn download(&self, url: &str) {
        let file_name = url.split('/').last().unwrap_or("");
        if !file_name.ends_with(".mp3") {
            return;
        }
        if !self.assets.audio_exists(file_name) {
            let res = if let Ok(data) = utils::download_audio(url).await {
                data
            } else {
                return;
            };
            if let Err(e) = self.assets.write_audio(file_name, res) {
                log::error!("Error: {}", e);
            }
        }
    }

    /// Load schedule.
    async fn load_schedule(&self) -> Vec<Schedule> {
        let repo = self.db.repository::<ScheduleRepo>();
        let schedules = repo.get().unwrap_or_default();
        let mut formated = vec![];
        for schedule in schedules {
            let schedule = Schedule {
                sid: schedule.sid,
                name: schedule.name,
                days: serde_json::from_str(&schedule.days).unwrap(),
                record_url: schedule.record_url,
                kind: schedule.kind,
                weeks: serde_json::from_str(&schedule.weeks).unwrap(),
                dates: serde_json::from_str(&schedule.dates).unwrap(),
                times: serde_json::from_str(&schedule.times).unwrap(),
                month: schedule.month,
                year: schedule.year,
                volume: schedule.volume.map(|v| v as f32),
            };
            self.download(&schedule.record_url).await;
            formated.push(schedule);
        }
        formated
    }

    /// play job.
    async fn play(&self, name: &str, url: &str, volume: Option<f32>) {
        log::info!("Try to Play: {} {}", name, url);
        if *self.block.read().await {
            log::error!("Can't play {} because schedule is blocked", name);
            return;
        }
        if self.player.is_playing() {
            log::error!("Can't play {} because other stream is playing", name);
            return;
        }

        let file_name = url.split('/').last().unwrap_or("");
        if !file_name.ends_with(".mp3") {
            log::error!("Can't play {} because it's not an mp3 file", name);
            return;
        }
        if !self.assets.audio_exists(file_name) {
            log::error!("Can't play {} because it doesn't exist", name);
            return;
        }
        let path = self.assets.audio_path(file_name);
        log::debug!("Assigning player");
        self.player.set_volume(volume.unwrap_or(1.0));
        let f = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Error: {}", e);
                return;
            }
        };
        let reader = BufReader::new(f);
        let decoder = match Decoder::new(reader) {
            Ok(d) => d,
            Err(e) => {
                log::error!("Error: {}", e);
                return;
            }
        };
        log::info!("Play: {} {}", name, url);
        self.player.append(decoder);
        self.player.play();
        log::debug!("Player assigned");
    }

    /// Set jobs.
    async fn set_jobs(&self, jobs: Vec<Schedule>) {
        log::info!("Setting jobs");
        let state = self.clone();
        let mut lock = self.runner.write().await;
        lock.push(tokio::spawn(async move {
            let state = state.clone();
            let jobs = jobs;
            let mut tick = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                tick.tick().await;
                let now = utils::time::time_24();
                for job in jobs.iter() {
                    let now_clone = now.clone();
                    let job = job.clone();
                    let state = state.clone();
                    tokio::spawn(async move {
                        let job = job.clone();
                        let now = now_clone;
                        let times = job.times.into_iter().flatten().collect::<Vec<String>>();
                        let weeks = job.weeks.into_iter().flatten().collect::<Vec<i32>>();
                        let days = job.days.into_iter().flatten().collect::<Vec<i32>>();
                        let dates = job.dates.into_iter().flatten().collect::<Vec<i32>>();

                        if times.contains(&now) {
                            match job.kind {
                                1 => {
                                    if weeks.contains(&(week_no() as i32))
                                        && days.contains(&(day_of_week() as i32))
                                    {
                                        state.play(&job.name, &job.record_url, job.volume).await;
                                    }

                                    if dates.contains(&(day_no() as i32)) {
                                        state.play(&job.name, &job.record_url, job.volume).await;
                                    }
                                }
                                2 => {
                                    let cm = current_month() as i32;
                                    let cy = current_year() as i32;
                                    if job.month.is_none() || job.year.is_none() {
                                        return;
                                    }
                                    let month = job.month.unwrap();
                                    let year = job.year.unwrap();
                                    if month == cm
                                        && year == cy
                                        && dates.contains(&(day_no() as i32))
                                    {
                                        state.play(&job.name, &job.record_url, job.volume).await;
                                    }
                                }
                                _ => {
                                    log::warn!("Invalid job kind: {}", job.kind);
                                }
                            }
                        } else {
                            log::debug!("Skipping job: {}", now);
                        }
                    });
                }
            }
        }));
    }

    /// Update schedule.
    pub fn update(&self) {
        log::info!("Update schedule");
        let state = self.clone();
        tokio::spawn(async move {
            let state = state.clone();
            let schedules = state.load_schedule().await;
            state.clear_runner().await;
            state.set_jobs(schedules).await;
        });
    }

    /// Block scheduler.
    pub async fn block(&self) {
        self.player.clear();
        *self.block.write().await = true;
    }

    /// Unblock scheduler.
    pub async fn unblock(&self) {
        self.player.clear();
        *self.block.write().await = false;
    }

    /// Run scheduler.
    pub async fn run(&self) {
        log::info!("Starting scheduler");
        if !self.runner.read().await.is_empty() {
            self.clear_runner().await;
        }
        let schedules = self.load_schedule().await;
        self.set_jobs(schedules).await;
    }
}
