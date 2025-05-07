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

use crate::ApiDatabase;
use database::{DatabaseRepository, Result};
use diesel::prelude::*;
use types::api::{
    avs::Avs,
    city::City,
    package::Package,
    province::Province,
    records::Records,
    role::Role,
    schedules::Schedules,
    schema::*,
    subscription::{NewSubscription, Subscription},
    user::{NewUser, UpdateUser, User},
    user_group::UserGroup,
};

/// UserRepo.
/// Repository for `user` table.
#[derive(Clone)]
pub struct UserRepo {
    db: ApiDatabase,
}

impl UserRepo {
    /// Get user by id.
    pub fn get(&self, id: i32) -> Result<User> {
        self.db.run(|conn| {
            let res = users::table.find(id).first::<User>(conn)?;
            Ok(res)
        })
    }

    /// Create a new user.
    pub fn create(&self, mut user: NewUser, package_id: i32) -> Result<User> {
        self.db.run_transaction(|conn| {
            let mut device_ids = vec![];
            for device in user.device_ids {
                if let Some(device) = device {
                    if device == 0 {
                        continue;
                    }
                    avs::table.find(device).first::<Avs>(conn)?;
                    device_ids.push(Some(device));
                }
            }
            let mut user_group_ids = vec![];
            for group in user.user_group_ids {
                if let Some(group) = group {
                    if group == 0 {
                        continue;
                    }
                    user_group::table.find(group).first::<UserGroup>(conn)?;
                    user_group_ids.push(Some(group));
                }
            }
            user.device_ids = device_ids;
            user.user_group_ids = user_group_ids;
            let res = diesel::insert_into(users::table)
                .values(&user)
                .get_result::<User>(conn)?;
            if user.role_id == 3 {
                let now = utils::time::now();
                let expired = utils::time::add_year(now.clone());
                diesel::insert_into(subscription::table)
                    .values(&NewSubscription {
                        user_id: res.id,
                        package_id,
                        order_date: now,
                        expire_date: expired,
                    })
                    .execute(conn)?;
            }
            Ok(res)
        })
    }

    /// Get user data by id.
    pub fn get_data(
        &self,
        id: i32,
    ) -> Result<(
        User,
        Option<(City, Province)>,
        Vec<UserGroup>,
        Role,
        Option<(Subscription, Package)>,
    )> {
        self.db.run(|conn| {
            let (user, role) = users::table
                .inner_join(role::table)
                .filter(users::id.eq(id))
                .first::<(User, Role)>(conn)?;
            let mut city_res: Option<(City, Province)> = None;
            let mut group_res: Vec<UserGroup> = vec![];
            if let Some(id) = user.city_id {
                city_res = city::table
                    .inner_join(province::table)
                    .filter(city::id.eq(id))
                    .first(conn)
                    .optional()?;
            };
            for g in &user.user_group_ids {
                if let Some(g) = g {
                    group_res.push(user_group::table.find(g).first::<UserGroup>(conn)?);
                }
            }
            let subscription_res: Option<(Subscription, Package)> = subscription::table
                .inner_join(package::table)
                .filter(subscription::user_id.eq(user.id))
                .first(conn)
                .optional()?;
            Ok((user, city_res, group_res, role, subscription_res))
        })
    }

    /// Get user by email.
    pub fn get_by_email(&self, email: &str) -> Result<User> {
        self.db.run(|conn| {
            let res = users::table
                .filter(users::email.eq(email))
                .first::<User>(conn)?;
            Ok(res)
        })
    }

    /// Update user.
    pub fn update(&self, mut user: UpdateUser) -> Result<User> {
        self.db.run(|conn| {
            let mut group_ids = vec![];
            for group in user.user_group_ids {
                if let Some(group) = group {
                    if group == 0 {
                        continue;
                    }
                    user_group::table.find(group).first::<UserGroup>(conn)?;
                    group_ids.push(Some(group));
                }
            }
            let mut device_ids = vec![];
            for device in user.device_ids {
                if let Some(device) = device {
                    if device == 0 {
                        continue;
                    }
                    avs::table.find(device).first::<Avs>(conn)?;
                    device_ids.push(Some(device));
                }
            }
            user.device_ids = device_ids;
            user.user_group_ids = group_ids;
            let res = diesel::update(users::table)
                .filter(users::id.eq(user.id))
                .set(user)
                .get_result::<User>(conn)?;
            Ok(res)
        })
    }

    /// Update user profile.
    pub fn update_profile(&self, user_id: i32, image_url: &str) -> Result<User> {
        self.db.run(|conn| {
            diesel::update(users::table)
                .filter(users::id.eq(user_id))
                .set(users::image_url.eq(image_url))
                .get_result::<User>(conn)
                .map_err(Into::into)
        })
    }

    /// Delete user.
    pub fn delete(&self, user_id: i32) -> Result<User> {
        self.db.run_transaction(|conn| {
            let users = users::table.find(user_id).first::<User>(conn)?;
            let subs = subscription::table
                .filter(subscription::user_id.eq(user_id))
                .first::<Subscription>(conn)
                .optional()?;
            let sched = schedules::table
                .filter(schedules::user_id.eq(user_id))
                .load::<Schedules>(conn)?;
            let recs = records::table
                .filter(records::user_id.eq(user_id))
                .load::<Records>(conn)?;
            if subs.is_some() {
                diesel::delete(subscription::table.filter(subscription::user_id.eq(user_id)))
                    .execute(conn)?;
            }
            if !sched.is_empty() {
                diesel::delete(schedules::table.filter(schedules::user_id.eq(user_id)))
                    .execute(conn)?;
            }
            if !recs.is_empty() {
                diesel::delete(records::table.filter(records::user_id.eq(user_id)))
                    .execute(conn)?;
            }
            diesel::delete(users::table.filter(users::id.eq(user_id))).execute(conn)?;
            Ok(users)
        })
    }
}

impl DatabaseRepository<PgConnection> for UserRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
