// Keg, the lightweight backend of the Musikverein Leopoldsdorf.
// Copyright (C) 2022  Richard Stöckl
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.

use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

use crate::schema_util::SchemaExample;

#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(example = "Self::example")]
/// Representation of a score considering the intellectual property, not a specific copy.
pub struct Score {
    /// The id of the score, should be generated by the dbms.
    #[serde(skip_deserializing)]
    pub id: Option<i64>,
    /// The main title of this score.
    pub title: String,
    /// The genres of the score.
    pub genres: Vec<String>,
    /// All composers of the score.
    /// The order is not considered here and every composer will only be persisted once per score and over the whole database.
    pub composers: Vec<String>,
    /// All arrangers of the score.
    /// The order is not considered here and every arranger will only be persisted once per score and over the whole database.
    pub arrangers: Vec<String>,
    /// All publishers of the score.
    /// The order is not considered here and every publisher will only be persisted once per score and over the whole database.
    pub publishers: Vec<String>,
    /// The grade of this score.
    pub grade: Option<String>,
    /// Other known titles for the scores.
    /// Often bohemian titles.
    /// The order is not considered here and every alias will only be persisted once.
    pub alias: Vec<String>,
    /// The subtitles of the score which can be used to mark smes, potpourri or medley.
    /// The order is mandatory here and every sub-title can occur more than once.
    pub sub_titles: Vec<String>,
    /// The annotation of this score.
    pub annotation: Option<String>,
    /// The score of the back of the one.
    pub back_of: Option<i64>,
    /// Where the score currently is located at.
    pub location: Option<i64>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(example = "Self::example")]
pub struct Book {
    /// The id of the book.
    #[serde(skip_deserializing)]
    pub id: Option<i64>,
    /// Full name of the book.
    pub name: String,
    /// Annotation of the book.
    pub annotation: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(example = "Self::example")]
/// A page which represents where a particular score is located in a book.
pub struct Page {
    /// The id of the book where this page is located at.
    pub book: i64,
    /// The number where the page begins at.
    pub begin: PageNumber,
    /// The number where the page ends at.
    /// The page ends at `begin` if absent.
    pub end: Option<PageNumber>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(example = "Self::example")]
/// A page-number.
pub struct PageNumber {
    /// The prefix of the page.
    pub prefix: Option<String>,
    /// The actual number of the page.
    pub number: i64,
    /// The suffix of the page.
    pub suffix: Option<String>,
}

#[derive(JsonSchema, FromForm, Debug)]
pub struct ScoreSearchParameters {
    pub fields: Vec<ScoreSearchParameter>,
    pub descending: bool,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, FromFormField)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[schemars(example = "Self::example")]
pub enum ScoreSearchParameter {
    Title,
    Genre,
    SubTitle,
    Arranger,
    Composer,
    Annotation,
    Alias,
    Publisher,
}

impl SchemaExample for Score {
    fn example() -> Self {
        Self {
            id: Some(42),
            title: "baum".to_string(),
            genres: vec![],
            composers: vec![],
            arrangers: vec![],
            publishers: vec![],
            grade: None,
            alias: vec!["strauch".to_string(), "teller".to_string()],
            sub_titles: vec![],
            annotation: None,
            back_of: None,
            location: None,
        }
    }
}

impl SchemaExample for PageNumber {
    fn example() -> Self {
        Self {
            prefix: Some("A".to_string()),
            number: 6,
            suffix: None,
        }
    }
}

impl SchemaExample for Page {
    fn example() -> Self {
        Self {
            book: 5,
            begin: Default::default(),
            end: None,
        }
    }
}

impl SchemaExample for Book {
    fn example() -> Self {
        Self {
            id: Some(5),
            name: "Rot".to_string(),
            annotation: Some("New covers".to_string()),
        }
    }
}

impl SchemaExample for ScoreSearchParameter {
    fn example() -> Self {
        Self::Title
    }
}

impl ScoreSearchParameter {
    fn to_database_field(&self) -> String {
        match self {
            ScoreSearchParameter::Title => "title",
            ScoreSearchParameter::SubTitle => "subtitle",
            ScoreSearchParameter::Alias => "alias",
            ScoreSearchParameter::Publisher => "publisher",
            ScoreSearchParameter::Annotation => "annotation",
            ScoreSearchParameter::Composer => "composer",
            ScoreSearchParameter::Arranger => "arranger",
            ScoreSearchParameter::Genre => "genre",
        }
        .to_string()
    }
}
