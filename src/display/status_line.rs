use std::str::FromStr;
use {
    crate::{
        app::Status,
        errors::ProgramError,
        skin::PanelSkin,
    },
    super::{Screen, W},
    termimad::{
        Area,
        minimad::{Alignment, Composite}, StyledChar,
    },
};
use chrono::{DateTime, Duration, Local, Utc};
static ELLIPSIS_CYCLE : Duration = Duration::milliseconds(100);
static mut CURRENT : usize = 0;
static mut LAST_UPDATE: Option<DateTime<Utc>> = None;
static chars: [&str; 4] = ["   ", ".  ", ".. ", "..."];
fn ellipsis(last_draw: DateTime<Utc>) -> &'static str {
    let now = Utc::now();
    let last_draw = now - last_draw;
    unsafe {
    let last = LAST_UPDATE.or(Some(Utc::now())).unwrap();
    let since_last = now - last;
    if ( since_last > ELLIPSIS_CYCLE ) || ( last_draw > ELLIPSIS_CYCLE) {
        LAST_UPDATE = Some(now);
        if CURRENT == 3 {
            CURRENT = 0;
            chars[CURRENT]
        } else {
            CURRENT += 1;
            chars[CURRENT]
        }
    } else {
        chars[CURRENT]
    }
    }
}

/// write the whole status line (task + status)
pub unsafe fn write(
    w: &mut W,
    task: Option<&str>,
    status: &Status,
    area: &Area,
    panel_skin: &PanelSkin,
    screen: Screen,
    last_redraw: DateTime<Utc>,
) -> Result<(), ProgramError> {
    let y = area.top;
    screen.goto(w, area.left, y)?;
    let mut x = area.left;
    if let Some(pending_task) = task {
        let mut spinner = spinners::Spinners::from_str("Dots9").unwrap();
        let pending_task = format!("{pending_task}{}", ellipsis(last_redraw));
        x += pending_task.chars().count() as u16;
        panel_skin.styles.status_job.queue(w, pending_task)?;
    }
    screen.goto(w, x, y)?;
    let style = if status.error {
        &panel_skin.status_skin.error
    } else {
        &panel_skin.status_skin.normal
    };
    style.write_inline_on(w, " ")?;
    let remaining_width = (area.width - (x - area.left) - 1) as usize;
    style.write_composite_fill(
        w,
        Composite::from_inline(&status.message),
        remaining_width,
        Alignment::Unspecified,
    )?;
    Ok(())
}

/// erase the whole status line
pub fn erase(
    w: &mut W,
    area: &Area,
    panel_skin: &PanelSkin,
    screen: Screen,
) -> Result<(), ProgramError> {
    screen.goto(w, area.left, area.top)?;
    let sc = StyledChar::new(
        panel_skin.status_skin.normal.paragraph.compound_style.clone(),
        ' ',
    );
    sc.queue_repeat(w, area.width as usize)?;
    Ok(())
}

