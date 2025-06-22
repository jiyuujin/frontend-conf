use chrono::{Duration, NaiveDate};
use js_sys::{Array, Date, Object};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, HtmlElement};

#[derive(Serialize, Deserialize, Clone)]
pub struct Conference {
  id: u32,
  name: String,
  location: String,
  region: String,
  date: String,
  status: String,
  description: String,
  attendees: String,
  venue: String,
  color_class: String,
  light_color_class: String,
  text_color_class: String,
  detail_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpcomingEvent {
  title: String,
  conference: String,
  date: String,
  event_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Stat {
  value: String,
  label: String,
  color: String,
}

#[wasm_bindgen]
pub struct ConferenceHub {
  conferences: Vec<Conference>,
  upcoming_events: Vec<UpcomingEvent>,
  stats: Vec<Stat>,
  document: Document,
}

#[wasm_bindgen]
impl ConferenceHub {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Result<ConferenceHub, JsValue> {
    console_error_panic_hook::set_once();

    let window = window().expect("グローバルウィンドウオブジェクトが見つかりません");
    let document = window.document().expect("現在のドキュメントが見つかりません");

    let conferences = vec![
      Conference {
        id: 2,
        name: "フロントエンドカンファレンス北海道2025".to_string(),
        location: "札幌".to_string(),
        region: "北海道".to_string(),
        date: "2025年9月6日".to_string(),
        status: "開催決定".to_string(),
        description: "2025年9月6日（土）に北海道札幌市で開催する、フロントエンド領域に関心のある参加者を対象とした技術イベントです。".to_string(),
        attendees: "100+".to_string(),
        venue: "エア・ウォーターの森".to_string(),
        color_class: "bg-green-500".to_string(),
        light_color_class: "bg-green-50 border-green-200".to_string(),
        text_color_class: "text-green-700".to_string(),
        detail_url: "https://note.com/fec_hokkaido".to_string(),
      },
      Conference {
        id: 3,
        name: "フロントエンドカンファレンス東京2025".to_string(),
        location: "東京".to_string(),
        region: "関東".to_string(),
        date: "2025年9月21日".to_string(),
        status: "開催決定".to_string(),
        description: "フロントエンドカンファレンス東京2025を、2025年9月21日（日）にAbema Towersにて開催いたします。".to_string(),
        attendees: "300+".to_string(),
        venue: "Abema Towers".to_string(),
        color_class: "bg-orange-500".to_string(),
        light_color_class: "bg-orange-50 border-orange-200".to_string(),
        text_color_class: "text-orange-700".to_string(),
        detail_url: "https://note.com/fec_tokyo".to_string(),
      },
      Conference {
        id: 1,
        name: "フロントエンドカンファレンス関西2025".to_string(),
        location: "大阪".to_string(),
        region: "関西".to_string(),
        date: "2025年11月30日".to_string(),
        status: "開催決定".to_string(),
        description: "出会いが共鳴し、次の誰かを動かす。2025年11月30日（日）大阪にて日本最大級のフロントエンドカンファレンス関西が開催！".to_string(),
        attendees: "300+".to_string(),
        venue: "マイドームおおさか".to_string(),
        color_class: "bg-purple-500".to_string(),
        light_color_class: "bg-purple-50 border-purple-200".to_string(),
        text_color_class: "text-purple-700".to_string(),
        detail_url: "https://medium.com/fec-kansai".to_string(),
      },
    ];

    let upcoming_events = vec![
      UpcomingEvent {
        title: "CfP募集開始".to_string(),
        conference: "フロントエンドカンファレンス北海道2025".to_string(),
        date: "2025年5月14日".to_string(),
        event_type: "募集".to_string(),
      },
      UpcomingEvent {
        title: "スポンサー募集開始".to_string(),
        conference: "フロントエンドカンファレンス北海道2025".to_string(),
        date: "2025年5月22日".to_string(),
        event_type: "募集".to_string(),
      },
      UpcomingEvent {
        title: "CfP募集開始".to_string(),
        conference: "フロントエンドカンファレンス東京2025".to_string(),
        date: "2025年5月28日".to_string(),
        event_type: "募集".to_string(),
      },
      UpcomingEvent {
        title: "スポンサー募集開始".to_string(),
        conference: "フロントエンドカンファレンス関西2025".to_string(),
        date: "2025年6月2日".to_string(),
        event_type: "募集".to_string(),
      },
      UpcomingEvent {
        title: "スポンサー募集開始".to_string(),
        conference: "フロントエンドカンファレンス東京2025".to_string(),
        date: "2025年6月11日".to_string(),
        event_type: "募集".to_string(),
      },
      UpcomingEvent {
        title: "CfP募集開始".to_string(),
        conference: "フロントエンドカンファレンス関西2025".to_string(),
        date: "2025年6月27日".to_string(),
        event_type: "募集".to_string(),
      },
    ];

    let stats = vec![
      Stat {
        value: "4".to_string(),
        label: "開催都市".to_string(),
        color: "text-blue-600".to_string(),
      },
      Stat {
        value: "2000+".to_string(),
        label: "総参加者数".to_string(),
        color: "text-green-600".to_string(),
      },
      Stat {
        value: "100+".to_string(),
        label: "セッション数".to_string(),
        color: "text-purple-600".to_string(),
      },
      Stat {
        value: "50+".to_string(),
        label: "スピーカー数".to_string(),
        color: "text-orange-600".to_string(),
      },
    ];

    Ok(ConferenceHub {
      conferences,
      upcoming_events,
      stats,
      document,
    })
  }

  #[wasm_bindgen]
  pub fn initialize(&self) -> Result<(), JsValue> {
    self.render_upcoming_events()?;
    self.render_conference_cards()?;
    self.render_stats()?;
    self.setup_event_listeners()?;
    Ok(())
  }

  fn render_upcoming_events(&self) -> Result<(), JsValue> {
    let container = self.document.get_element_by_id("upcomingEvents").ok_or_else(|| JsValue::from_str("upcomingEventsコンテナが見つかりません"))?;

    let mut html = String::new();

    for event in &self.upcoming_events {
      let badge_class = match event.event_type.as_str() {
        "募集" => "bg-blue-500 text-white",
        "チケット" => "bg-gray-500 text-white",
        _ => "bg-gray-200 text-gray-800",
      };

      html.push_str(&format!(
        r#"
          <div class="bg-white border border-gray-200 rounded-lg p-4 border-l-4 border-l-blue-500 card-hover">
            <div class="flex items-center justify-between mb-2">
              <span class="{} text-sm px-3 py-1 rounded-full">{}</span>
              <span class="text-sm text-gray-500">{}</span>
            </div>
            <h4 class="font-semibold text-gray-900 mb-1">{}</h4>
            <p class="text-sm text-gray-600">{}</p>
          </div>
        "#,
        badge_class, event.event_type, event.date, event.title, event.conference
      ));
    }

    container.set_inner_html(&html);
    Ok(())
  }

  fn render_conference_cards(&self) -> Result<(), JsValue> {
    let container = self.document.get_element_by_id("conferenceCards").ok_or_else(|| JsValue::from_str("conferenceCardsコンテナが見つかりません"))?;

    let mut html = String::new();

    for conf in &self.conferences {
      let status_badge_class = if conf.status == "開催決定" {
        "bg-green-500 text-white"
      } else {
        "bg-gray-500 text-white"
      };

      html.push_str(&format!(
        r#"
          <div class="bg-white border rounded-lg {} card-hover">
            <div class="p-6">
              <div class="flex items-center justify-between mb-4">
                <span class="{} text-white text-sm px-3 py-1 rounded-full">{}</span>
                <span class="{} text-sm px-3 py-1 rounded-full">{}</span>
              </div>
              <h3 class="text-xl font-bold text-gray-900 mb-2">{}</h3>
              <p class="{} mb-4">{}</p>
              <div class="space-y-3 mb-4">
                <div class="flex items-center text-gray-600">
                  <i data-lucide="calendar" class="mr-2 h-4 w-4"></i>
                  <span>{}</span>
                </div>
                <div class="flex items-center text-gray-600">
                  <i data-lucide="map-pin" class="mr-2 h-4 w-4"></i>
                  <span>{} ({})</span>
                </div>
                <div class="flex items-center text-gray-600">
                  <i data-lucide="users" class="mr-2 h-4 w-4"></i>
                  <span>予想参加者数: {}</span>
                </div>
              </div>
              <div class="flex space-x-2">
                <a
                  href="{}" 
                  target="_blank" 
                  rel="noopener noreferrer"
                  class="flex-1 bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 transition-colors flex items-center justify-center text-decoration-none"
                >
                  <i data-lucide="external-link" class="mr-2 h-4 w-4"></i>
                  詳細を見る
                </a>
                <button
                  class="border border-gray-300 text-gray-700 px-4 py-2 rounded-lg hover:bg-gray-50 transition-colors flex items-center justify-center"
                  data-conference-id="{}"
                  data-action="calendar"
                >
                  <i data-lucide="calendar-plus" class="mr-2 h-4 w-4"></i>
                  Googleカレンダーに追加する
                </button>
              </div>
            </div>
          </div>
        "#,
        conf.light_color_class, conf.color_class, conf.region, status_badge_class, conf.status,
        conf.name, conf.text_color_class, conf.description, conf.date, conf.venue, conf.location,
        conf.attendees, conf.detail_url, conf.id
      ));
    }

    container.set_inner_html(&html);

    let window = window().expect("グローバルウィンドウオブジェクトが見つかりません");
    let _ = js_sys::eval("if (window.lucide) window.lucide.createIcons();");

    Ok(())
  }

  fn render_stats(&self) -> Result<(), JsValue> {
    let container = self.document.get_element_by_id("statsSection").ok_or_else(|| JsValue::from_str("statsSectionコンテナが見つかりません"))?;

    let mut html = String::new();

    for stat in &self.stats {
      html.push_str(&format!(
        r#"
          <div>
            <div class="text-3xl font-bold {} mb-2">{}</div>
            <div class="text-gray-600">{}</div>
          </div>
        "#,
        stat.color, stat.value, stat.label
      ));
    }

    container.set_inner_html(&html);
    Ok(())
  }

  fn setup_event_listeners(&self) -> Result<(), JsValue> {
    let document_clone = self.document.clone();
    let conferences_clone = self.conferences.clone();

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
      let target = event.target().unwrap();
      let element = target.dyn_into::<web_sys::Element>().unwrap();

      let button = if element.tag_name() == "BUTTON" {
        element
      } else {
        let parent = element.closest("button").unwrap();
        if parent.is_some() {
          parent.unwrap()
        } else {
          return;
        }
      };

      if let Some(action) = button.get_attribute("data-action") {
        match action.as_str() {
          "calendar" => {
            if let Some(id_str) = button.get_attribute("data-conference-id") {
              if let Ok(id) = id_str.parse::<u32>() {
                if let Some(conference) = conferences_clone.iter().find(|c| c.id == id) {
                  let calendar_url = generate_google_calendar_url(conference);

                  let window = window().unwrap();
                  window.open_with_url_and_target(&calendar_url, "_blank").unwrap();

                  show_toast(&format!("{}をGoogleカレンダーに追加します", conference.name));
                }
              }
            }
          }
          _ => {}
        }
      }
    }) as Box<dyn FnMut(_)>);

    let document_web = &self.document;
    document_web.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;

    closure.forget();

    Ok(())
  }
}

fn generate_google_calendar_url(conference: &Conference) -> String {
  let base_url = "https://calendar.google.com/calendar/render?action=TEMPLATE";

  let title = js_sys::encode_uri_component(&conference.name);

  let dates = parse_conference_date(&conference.date);

  let details = js_sys::encode_uri_component(&format!(
    "{}\n\n会場: {}\n予想参加者数: {}\nステータス: {}\n\n詳細: {}",
    conference.description, conference.venue, conference.attendees, conference.status, conference.detail_url
  ));

  let location = js_sys::encode_uri_component(&format!("{}, {}", conference.venue, conference.location));

  format!(
    "{}&text={}&dates={}&details={}&location={}&ctz=Asia/Tokyo",
    base_url, title, dates, details, location
  )
}

fn parse_conference_date(date_str: &str) -> String {
  let re = regex::Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日(?:-(\d{1,2})日)?").unwrap();

  if let Some(caps) = re.captures(date_str) {
    let year: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let month: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let start_day: u32 = caps.get(3).unwrap().as_str().parse().unwrap();

    let start_date = NaiveDate::from_ymd_opt(year, month, start_day).unwrap();

    let end_date = if let Some(end_day_match) = caps.get(4) {
      let end_day: u32 = end_day_match.as_str().parse().unwrap();
      let end = NaiveDate::from_ymd_opt(year, month, end_day).unwrap();
      end + Duration::days(1) // 終了日の翌日
    } else {
      start_date + Duration::days(1) // 1日イベントの場合
    };

    let format_date = |date: NaiveDate| format!("{}T000000Z", date.format("%Y%m%d"));

    format!("{}/{}", format_date(start_date), format_date(end_date))
  } else {
    let today = chrono::Local::now().naive_local().date();
    let tomorrow = today + Duration::days(1);

    let format_date = |date: NaiveDate| format!("{}T000000Z", date.format("%Y%m%d"));

    format!("{}/{}", format_date(today), format_date(tomorrow))
  }
}

fn parse_japanese_date(date_str: &str) -> Option<NaiveDate> {
  let re = regex::Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日").unwrap();

  if let Some(caps) = re.captures(date_str) {
    let year: i32 = caps.get(1)?.as_str().parse().ok()?;
    let month: u32 = caps.get(2)?.as_str().parse().ok()?;
    let day: u32 = caps.get(3)?.as_str().parse().ok()?;

    NaiveDate::from_ymd_opt(year, month, day)
  } else {
    None
  }
}

fn show_toast(message: &str) {
  let window = window().expect("グローバルウィンドウオブジェクトが見つかりません");
  let _ = js_sys::eval(&format!(
    "
      const toast = document.getElementById('toast');
      const toastMessage = document.getElementById('toastMessage');
      if (toast && toastMessage) {{
        toastMessage.textContent = '{}';
        toast.classList.remove('translate-x-full');
        setTimeout(() => {{
          toast.classList.add('translate-x-full');
        }}, 3000);
      }}
    ",
    message
  ));
}
