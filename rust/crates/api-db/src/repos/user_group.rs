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
    city::City,
    package::{Package, PackageResponse},
    province::{Province, ProvinceResponse},
    role::{Role, RoleResponse},
    schema::*,
    subscription::Subscription,
    user::User,
    user_group::{NewUserGroup, UpdateUserGroup, UserGroup},
    CityData, GroupData, SubscriptionData, UserData,
};

fn count_nested(g: GroupData) -> i64 {
    let mut count = 0;
    if g.children.len() > 0 {
        for c in g.children {
            count += 1;
            count += count_nested(c);
        }
    }
    count
}

/// UserGroupRepo.
/// Repository for `user_group` table.
#[derive(Clone)]
pub struct UserGroupRepo {
    db: ApiDatabase,
}

impl UserGroupRepo {
    /// Filter by user.
    fn filter_by_user(&self, user_id: i32) -> Result<(Vec<UserGroup>, bool)> {
        self.db.run(|conn| {
            let user = users::table.find(user_id).first::<User>(conn)?;
            if user.role_id == 1 || user.role_id == 2 {
                let res = user_group::table
                    .order(user_group::name.asc())
                    .load::<UserGroup>(conn)?;
                Ok((res, true))
            } else {
                let mut ids = Vec::new();
                for id in user.user_group_ids {
                    if let Some(id) = id {
                        ids.push(id);
                    }
                }
                let groups = user_group::table
                    .order(user_group::name.asc())
                    .filter(user_group::id.eq_any(ids))
                    .load::<UserGroup>(conn)?;
                Ok((groups, false))
            }
        })
    }

    /// Get all user groups from seeds.
    fn get_all_from_seeds(&self, group_ids: Vec<i32>) -> Result<Vec<UserGroup>> {
        self.db.run(|conn| {
            let mut res = user_group::table
                .filter(user_group::parent_id.eq_any(group_ids))
                .load::<UserGroup>(conn)?;
            if !res.is_empty() {
                let ids = res.iter().map(|g| g.id).collect::<Vec<i32>>();
                let next = self.get_all_from_seeds(ids)?;
                res.extend(next);
            }
            Ok(res)
        })
    }

    /// Get all user groups.
    pub fn get_all(&self, user_id: i32) -> Result<Vec<UserGroup>> {
        let (seed, skip) = self.filter_by_user(user_id)?;
        if skip {
            Ok(seed)
        } else {
            let ids = seed.iter().map(|g| g.id).collect::<Vec<i32>>();
            self.get_all_from_seeds(ids)
        }
    }

    /// Get user data that doesn't joined any group.
    fn get_user_no_group(&self) -> Result<Vec<UserData>> {
        self.db.run(|conn| {
            let user = users::table
                .inner_join(role::table)
                .left_join(city::table.inner_join(province::table))
                .left_join(subscription::table.inner_join(package::table))
                .filter(users::user_group_ids.eq(Vec::<Option<i32>>::new()))
                .load::<(
                    User,
                    Role,
                    Option<(City, Province)>,
                    Option<(Subscription, Package)>,
                )>(conn)?;
            let user = user
                .into_iter()
                .map(|(u, r, c, s)| {
                    UserData::new(
                        u.id,
                        u.name,
                        u.email,
                        u.image_url,
                        RoleResponse::new(r.id, r.name),
                        c.map(|(c, p)| {
                            CityData::new(c.id, c.name, ProvinceResponse::new(p.id, p.name))
                        }),
                        vec![],
                        s.map(|(s, p)| {
                            SubscriptionData::new(
                                s.id,
                                s.user_id,
                                PackageResponse::new(p.id, p.name, p.max_devices),
                                s.order_date,
                                s.expire_date,
                            )
                        }),
                        u.device_ids,
                        u.user_group_ids,
                    )
                })
                .collect::<Vec<UserData>>();
            Ok(user)
        })
    }

    /// Get user data from given group_ids.
    fn get_user_data(&self, group_ids: Vec<i32>) -> Result<Vec<(i32, Vec<UserData>)>> {
        self.db.run(|conn| {
            let mut res = vec![];
            for id in group_ids {
                let user = users::table
                    .inner_join(role::table)
                    .left_join(city::table.inner_join(province::table))
                    .left_join(subscription::table.inner_join(package::table))
                    .filter(users::user_group_ids.contains(vec![Some(id)]))
                    .load::<(
                        User,
                        Role,
                        Option<(City, Province)>,
                        Option<(Subscription, Package)>,
                    )>(conn)?;
                let user = user
                    .into_iter()
                    .map(|(u, r, c, s)| {
                        UserData::new(
                            u.id,
                            u.name,
                            u.email,
                            u.image_url,
                            RoleResponse::new(r.id, r.name),
                            c.map(|(c, p)| {
                                CityData::new(c.id, c.name, ProvinceResponse::new(p.id, p.name))
                            }),
                            vec![],
                            s.map(|(s, p)| {
                                SubscriptionData::new(
                                    s.id,
                                    s.user_id,
                                    PackageResponse::new(p.id, p.name, p.max_devices),
                                    s.order_date,
                                    s.expire_date,
                                )
                            }),
                            u.device_ids,
                            u.user_group_ids,
                        )
                    })
                    .collect::<Vec<UserData>>();
                res.push((id, user));
            }
            Ok(res)
        })
    }

    /// Get related group from seeds.
    fn get_related_from_seeds(&self, group_ids: Vec<i32>) -> Result<Vec<GroupData>> {
        self.db.run(|conn| {
            let groups = user_group::table
                .filter(user_group::parent_id.eq_any(group_ids))
                .load::<UserGroup>(conn)?;
            let gids = groups.iter().map(|g| g.id).collect::<Vec<i32>>();
            let users = self.get_user_data(gids.clone())?;
            let children = if gids.is_empty() {
                vec![]
            } else {
                self.get_related_from_seeds(gids)?
            };
            let mut res = vec![];
            for g in groups {
                let user = users
                    .iter()
                    .find_map(|(id, u)| if *id == g.id { Some(u) } else { None })
                    .cloned()
                    .unwrap_or_default();
                let children = children
                    .iter()
                    .filter(|c| c.parent_id == Some(g.id))
                    .map(|c| c.clone())
                    .collect::<Vec<GroupData>>();
                res.push(GroupData::new(
                    g.id,
                    g.name,
                    g.description,
                    children,
                    user,
                    g.parent_id,
                ));
            }
            Ok(res)
        })
    }

    /// Get all user groups with it's relation paginated.
    pub fn get_all_related_paginated(
        &self,
        user_id: i32,
        page: i64,
    ) -> Result<(Vec<GroupData>, i64, i64)> {
        self.db.run(|conn| {
            let user = users::table.find(user_id).first::<User>(conn)?;
            if user.role_id == 1 || user.role_id == 2 {
                let groups = user_group::table
                    .order(user_group::name.asc())
                    .limit(10)
                    .offset((page - 1) * 10)
                    .filter(user_group::parent_id.is_null())
                    .load::<UserGroup>(conn)?;
                let total = user_group::table.load::<UserGroup>(conn)?.len() as i64;
                let total_pages = (total - 1) / 10 + 1;
                let gid = groups.iter().map(|g| g.id).collect::<Vec<i32>>();
                let user = self.get_user_data(gid.clone())?;
                let children = self.get_related_from_seeds(gid)?;
                let mut res = vec![];
                if page == 1 {
                    res.push(GroupData::new(
                        0,
                        "_".to_owned(),
                        None,
                        vec![],
                        self.get_user_no_group()?,
                        None,
                    ));
                }
                for g in groups {
                    let user = user
                        .iter()
                        .find_map(|(id, u)| if *id == g.id { Some(u) } else { None })
                        .cloned()
                        .unwrap_or_default();
                    let children = children
                        .iter()
                        .filter(|c| c.parent_id == Some(g.id))
                        .map(|c| c.clone())
                        .collect::<Vec<GroupData>>();
                    res.push(GroupData::new(
                        g.id,
                        g.name,
                        g.description,
                        children,
                        user,
                        g.parent_id,
                    ));
                }
                Ok((res, page, total_pages))
            } else {
                let user = users::table.find(user_id).first::<User>(conn)?;
                let mut ids = Vec::new();
                for id in user.user_group_ids {
                    if let Some(id) = id {
                        ids.push(id);
                    }
                }
                let groups = self.get_related_from_seeds(ids)?;
                Ok((groups, page, 1))
            }
        })
    }

    /// Count user group.
    pub fn count(&self, user_id: i32) -> Result<i64> {
        self.db.run(|conn| {
            let user = users::table.find(user_id).first::<User>(conn)?;
            if user.role_id == 1 || user.role_id == 2 {
                let groups = user_group::table
                    .order(user_group::name.asc())
                    .filter(user_group::parent_id.is_null())
                    .load::<UserGroup>(conn)?;
                let gid = groups.iter().map(|g| g.id).collect::<Vec<i32>>();
                let user = self.get_user_data(gid.clone())?;
                let children = self.get_related_from_seeds(gid)?;
                let mut res = vec![];
                for g in groups {
                    let user = user
                        .iter()
                        .find_map(|(id, u)| if *id == g.id { Some(u) } else { None })
                        .cloned()
                        .unwrap_or_default();
                    let children = children
                        .iter()
                        .filter(|c| c.parent_id == Some(g.id))
                        .map(|c| c.clone())
                        .collect::<Vec<GroupData>>();
                    res.push(GroupData::new(
                        g.id,
                        g.name,
                        g.description,
                        children,
                        user,
                        g.parent_id,
                    ));
                }
                let mut count = 0;
                for g in res {
                    count += 1;
                    count += count_nested(g);
                }
                Ok(count)
            } else {
                let user = users::table.find(user_id).first::<User>(conn)?;
                let mut ids = Vec::new();
                for id in user.user_group_ids {
                    if let Some(id) = id {
                        ids.push(id);
                    }
                }
                let groups = self.get_related_from_seeds(ids)?;
                let mut count = 0;
                for g in groups {
                    count += 1;
                    count += count_nested(g);
                }
                Ok(count)
            }
        })
    }

    /// Create new user group.
    pub fn create(&self, new: NewUserGroup) -> Result<UserGroup> {
        self.db.run(|conn| {
            if let Some(parent_id) = new.parent_id {
                let _ = user_group::table.find(parent_id).first::<UserGroup>(conn)?;
            }
            diesel::insert_into(user_group::table)
                .values(&new)
                .get_result::<UserGroup>(conn)
                .map_err(Into::into)
        })
    }

    /// Update user group.
    pub fn update(&self, update: UpdateUserGroup) -> Result<UserGroup> {
        self.db.run(|conn| {
            if let Some(parent_id) = update.parent_id {
                let _ = user_group::table.find(parent_id).first::<UserGroup>(conn)?;
            }
            diesel::update(user_group::table.find(update.id))
                .set(&update)
                .get_result::<UserGroup>(conn)
                .map_err(Into::into)
        })
    }
}

impl DatabaseRepository<PgConnection> for UserGroupRepo {
    fn new(db: database::Database<PgConnection>) -> Self {
        Self { db }
    }
}
