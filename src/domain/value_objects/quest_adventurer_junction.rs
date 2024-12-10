﻿use diesel::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::infrastructure::postgres::schema::quest_adventurer_junction;
use super::super::entities::{
    adventurers::AdventurerEntity,
    quests::QuestEntity,
};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, Queryable, Associations)]
#[diesel(belongs_to(QuestEntity, foreign_key = quest_id))]
#[diesel(belongs_to(AdventurerEntity, foreign_key = adventurer_id))]
#[diesel(table_name = quest_adventurer_junction)]
pub struct QuestAdventurerJunction {
    pub quest_id: i32,
    pub adventurer_id: i32,
}