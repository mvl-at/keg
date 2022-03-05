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

use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::archive::model::{Book, Score, ScoreSearchParameters};
use crate::schema_util;
use crate::schema_util::SchemaExample;

#[openapi(tag = "Archive")]
#[get("/score?<params..>")]
pub fn search_scores(params: ScoreSearchParameters) -> Json<schema_util::Page<Score>> {
    Json(schema_util::Page::example())
}

#[openapi(tag = "Archive")]
#[get("/score/<id>")]
pub fn get_score(id: i64) -> Json<Score> {
    Json(Score::example())
}

#[openapi(tag = "Archive")]
#[put("/score/<id>", data = "<score>")]
pub fn put_score(id: i64, score: Json<Score>) -> Json<Score> {
    Json(Score::example())
}

#[openapi(tag = "Archive")]
#[delete("/score/<id>")]
pub fn delete_score(id: i64) {}

#[openapi(tag = "Archive")]
#[post("/book", data = "<book>")]
pub fn post_book(book: Json<Book>) -> Json<Book> {
    Json(Book::example())
}
