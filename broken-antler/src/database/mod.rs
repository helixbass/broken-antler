use std::collections::HashMap;
use std::str::FromStr;

use derive_builder::Builder;
use itertools::Itertools;
use juriji::read_events;
use shared::Event;
use smallvec::SmallVec;
use smol_str::StrExt;
use sqlx::{Pool, Postgres};
use squalid::{_d, regex};
use tracing::instrument;
use trie_rs::map::Trie;
use uuid::Uuid;

use crate::{Set, Show, Song, Venue};

#[derive(Default)]
pub struct VenuesCobbler {
    pub assembling: Vec<Venue>,
}

impl VenuesCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertVenue(venue) => {
                self.assembling.push(venue.clone());
            }
            _ => {}
        }
    }
}

impl From<VenuesCobbler> for Vec<Venue> {
    fn from(value: VenuesCobbler) -> Self {
        value.assembling
    }
}

#[derive(Default)]
pub struct SongsCobbler {
    pub assembling: Vec<Song>,
}

impl SongsCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertSong(song) => {
                self.assembling.push(song.clone());
            }
            _ => {}
        }
    }
}

impl From<SongsCobbler> for Vec<Song> {
    fn from(value: SongsCobbler) -> Self {
        value.assembling
    }
}

#[derive(Default)]
pub struct ShowsCobbler {
    pub assembling: Vec<Show>,
}

impl ShowsCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertShow(show) => {
                self.assembling.push(show.clone());
            }
            _ => {}
        }
    }
}

impl From<ShowsCobbler> for Vec<Show> {
    fn from(value: ShowsCobbler) -> Self {
        value.assembling
    }
}

#[derive(Default)]
pub struct SetsCobbler {
    pub assembling: Vec<Set>,
}

impl SetsCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        match event {
            Event::InsertSet(set) => {
                self.assembling.push(set.clone());
            }
            _ => {}
        }
    }
}

impl From<SetsCobbler> for Vec<Set> {
    fn from(value: SetsCobbler) -> Self {
        value.assembling
    }
}

#[derive(Builder)]
pub struct Database {
    #[builder(setter(skip), default = "self.default_venues_by_id()")]
    pub venues_by_id: HashMap<Uuid, usize>,
    #[builder(setter(skip), default = "self.default_songs_by_id()")]
    pub songs_by_id: HashMap<Uuid, usize>,
    #[builder(setter(skip), default = "self.default_shows_by_id()")]
    pub shows_by_id: HashMap<Uuid, usize>,
    #[builder(setter(skip), default = "self.default_shows_by_venue_id()")]
    pub shows_by_venue_id: HashMap<Uuid, Vec<usize>>,
    #[builder(setter(skip), default = "self.default_sets_by_id()")]
    pub sets_by_id: HashMap<Uuid, usize>,
    #[builder(setter(skip), default = "self.default_venue_and_song_words()")]
    pub venue_and_song_words: Trie<u8, Word>,
    #[builder(setter(into))]
    pub venues: Vec<Venue>,
    #[builder(setter(into))]
    pub songs: Vec<Song>,
    #[builder(setter(into))]
    pub shows: Vec<Show>,
    #[builder(setter(into))]
    pub sets: Vec<Set>,
}

impl DatabaseBuilder {
    fn default_venues_by_id(&self) -> HashMap<Uuid, usize> {
        self.venues
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, venue)| (venue.id, index))
            .collect()
    }

    fn default_songs_by_id(&self) -> HashMap<Uuid, usize> {
        self.songs
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, song)| (song.id, index))
            .collect()
    }

    fn default_shows_by_id(&self) -> HashMap<Uuid, usize> {
        self.shows
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, show)| (show.id, index))
            .collect()
    }

    fn default_shows_by_venue_id(&self) -> HashMap<Uuid, Vec<usize>> {
        self.shows
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .into_group_map_by(|(_index, show)| show.venue_id)
            .into_iter()
            .map(|(venue_id, shows_and_indexes)| {
                (
                    venue_id,
                    shows_and_indexes
                        .into_iter()
                        .map(|(index, _)| index)
                        .collect::<Vec<_>>(),
                )
            })
            .collect()
    }

    fn default_sets_by_id(&self) -> HashMap<Uuid, usize> {
        self.sets
            .as_ref()
            .unwrap()
            .into_iter()
            .enumerate()
            .map(|(index, set)| (set.id, index))
            .collect()
    }

    fn default_venue_and_song_words(&self) -> Trie<u8, Word> {
        unimplemented!()
    }
}

impl Database {
    pub fn venue_by_id(&self, id: &Uuid) -> &Venue {
        &self.venues[self.venues_by_id[id]]
    }

    pub fn song_by_id(&self, id: &Uuid) -> &Song {
        &self.songs[self.songs_by_id[id]]
    }

    pub fn show_by_id(&self, id: &Uuid) -> &Show {
        &self.shows[self.shows_by_id[id]]
    }

    pub fn set_by_id(&self, id: &Uuid) -> &Set {
        &self.sets[self.sets_by_id[id]]
    }

    pub fn search_results<'a>(&'a self, query: &str) -> SearchResults<'a> {
        let query_words = regex!(r#"[^a-zA-Z0-9']+"#)
            .split(query)
            .collect::<SmallVec<[_; 10]>>();
        let months = query_words
            .iter()
            .enumerate()
            .filter_map(|(index, query_word)| {
                Month::from_str(query_word).ok().map(|month| (index, month))
            })
            .collect::<SmallVec<[_; 4]>>();
        let years = query_words
            .iter()
            .enumerate()
            .filter_map(|(index, query_word)| {
                Year::from_str(query_word).ok().map(|year| (index, year))
            })
            .collect::<SmallVec<[_; 4]>>();
        let days = query_words
            .iter()
            .enumerate()
            .filter_map(|(index, query_word)| {
                DayOfMonth::from_str(query_word)
                    .ok()
                    .map(|day| (index, day))
            })
            .collect::<SmallVec<[_; 4]>>();
        let all_show_dates = get_all_show_dates(&months, &days, &years);
        unimplemented!()
    }
}

type ShowDates = SmallVec<[ShowDate; 4]>;

fn get_all_show_dates(
    months: &[(usize, Month)],
    days: &[(usize, DayOfMonth)],
    years: &[(usize, Year)],
) -> SmallVec<[ShowDate; 4]> {
    let mut ret: ShowDates = _d();
    months.into_iter().for_each(|(month_index, month)| {
        days.into_iter()
            .filter(|(day_index, _)| day_index != month_index)
            .for_each(|(day_index, day)| {
                years
                    .into_iter()
                    .filter(|(year_index, _)| year_index != month_index && year_index != day_index)
                    .for_each(|(_, year)| {
                        ret.push(ShowDate::MonthAndDayAndYear {
                            month: *month,
                            day: *day,
                            year: *year,
                        });
                    });
            });
    });
    if !ret.is_empty() {
        return ret;
    }
    months.into_iter().for_each(|(month_index, month)| {
        days.into_iter()
            .filter(|(day_index, _)| day_index != month_index)
            .for_each(|(_, day)| {
                ret.push(ShowDate::MonthAndDay {
                    month: *month,
                    day: *day,
                });
            });
    });
    months.into_iter().for_each(|(month_index, month)| {
        years
            .into_iter()
            .filter(|(year_index, _)| year_index != month_index)
            .for_each(|(_, year)| {
                ret.push(ShowDate::MonthAndYear {
                    month: *month,
                    year: *year,
                });
            });
    });
    ret
}

pub enum Word {
    Venue { index: usize, venue_id: Uuid },
    Song { index: usize, song_id: Uuid },
}

pub enum SearchResult<'a> {
    Venue(&'a Venue),
    Show(&'a Show),
    Song(&'a Song),
}

impl<'a> From<&'a Venue> for SearchResult<'a> {
    fn from(value: &'a Venue) -> Self {
        Self::Venue(value)
    }
}

impl<'a> From<&'a Show> for SearchResult<'a> {
    fn from(value: &'a Show) -> Self {
        Self::Show(value)
    }
}

impl<'a> From<&'a Song> for SearchResult<'a> {
    fn from(value: &'a Song) -> Self {
        Self::Song(value)
    }
}

pub type SearchResults<'a> = SmallVec<[SearchResult<'a>; 16]>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl FromStr for Month {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match &*str.to_lowercase_smolstr() {
            "1" | "01" | "jan" | "january" => Ok(Self::January),
            "2" | "02" | "feb" | "february" => Ok(Self::February),
            "3" | "03" | "mar" | "march" => Ok(Self::March),
            "4" | "04" | "apr" | "april" => Ok(Self::April),
            "5" | "05" | "may" => Ok(Self::May),
            "6" | "06" | "jun" | "june" => Ok(Self::June),
            "7" | "07" | "jul" | "july" => Ok(Self::July),
            "8" | "08" | "aug" | "august" => Ok(Self::August),
            "9" | "09" | "sep" | "sept" | "september" => Ok(Self::September),
            "10" | "oct" | "october" => Ok(Self::October),
            "11" | "nov" | "november" => Ok(Self::November),
            "12" | "dec" | "december" => Ok(Self::December),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Year(u32);

impl FromStr for Year {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if !regex!(r#"^\d{4}$"#).is_match(str) {
            return Err(());
        }
        let year = str.parse::<u32>().unwrap();
        if year < 1980 {
            return Err(());
        }
        if year > 2040 {
            return Err(());
        }
        Ok(Self(year))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct DayOfMonth(u32);

impl FromStr for DayOfMonth {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if !regex!(r#"^\d\d?$"#).is_match(str) {
            return Err(());
        }
        let day = str.parse::<u32>().unwrap();
        if day < 1 {
            return Err(());
        }
        if day > 31 {
            return Err(());
        }
        Ok(Self(day))
    }
}

enum ShowDate {
    MonthAndDay {
        month: Month,
        day: DayOfMonth,
    },
    MonthAndYear {
        month: Month,
        year: Year,
    },
    MonthAndDayAndYear {
        month: Month,
        day: DayOfMonth,
        year: Year,
    },
}

#[derive(Default)]
pub struct DatabaseCobbler {
    pub venues: VenuesCobbler,
    pub songs: SongsCobbler,
    pub shows: ShowsCobbler,
    pub sets: SetsCobbler,
}

impl DatabaseCobbler {
    #[instrument(level = "trace", skip(self))]
    pub fn accept_next(&mut self, event: &Event) {
        self.venues.accept_next(event);
        self.songs.accept_next(event);
        self.shows.accept_next(event);
        self.sets.accept_next(event);
    }
}

impl From<DatabaseCobbler> for Database {
    fn from(value: DatabaseCobbler) -> Self {
        DatabaseBuilder::default()
            .venues(value.venues)
            .songs(value.songs)
            .shows(value.shows)
            .sets(value.sets)
            .build()
            .unwrap()
    }
}

#[instrument(level = "trace", skip(db_pool))]
pub async fn get_database(db_pool: &Pool<Postgres>) -> sauvignon::Database {
    let mut cobbler = DatabaseCobbler::default();
    read_events(None, db_pool)
        .await
        .into_iter()
        .map(|event| Event::from(&event))
        .for_each(|event| {
            cobbler.accept_next(&event);
        });
    sauvignon::Database::Dyn(Box::new(Database::from(cobbler)))
}
