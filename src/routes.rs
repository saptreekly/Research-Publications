//! Optional route groups registered as `#[component(transparent)]` wrappers.
//! `<Routes>` must only see `<Route/>` or transparent route components — never
//! `#[cfg]` attributes directly on `<Route/>` children.

use leptos::*;
use leptos_router::*;
use research_publications::pages::layout::RootLayout;

#[cfg(feature = "lab")]
use research_publications::pages::lab::LabPage;

#[cfg(feature = "malware-traffic")]
use research_publications::pages::malware_traffic::{MalwareTrafficIndexPage, MalwareTrafficPage};

#[cfg(feature = "situation-monitor")]
use research_publications::pages::situation_monitor::SituationMonitorPage;

#[cfg(feature = "tidy-tuesday")]
use research_publications::pages::tidy_tuesday::{TidyTuesdayIndexPage, TidyTuesdayPage};

const ROUTE_LAB: &str = "/Research-Publications/curriculum/lab/:slug";
const ROUTE_TIDY_TUESDAY: &str = "/Research-Publications/tidy-tuesday";
const ROUTE_TIDY_TUESDAY_ENTRY: &str = "/Research-Publications/tidy-tuesday/:slug";
const ROUTE_SITUATION_MONITOR: &str = "/Research-Publications/situation-monitor";
const ROUTE_MALWARE_REPORTS: &str = "/Research-Publications/malware-reports";
const ROUTE_MALWARE_REPORTS_ENTRY: &str = "/Research-Publications/malware-reports/:slug";

#[allow(unused_macros)]
macro_rules! noop_route {
    ($suffix:literal) => {
        view! {
            <Route
                path=concat!("/Research-Publications/__disabled/", $suffix)
                view=move || view! { <span style="display:none" aria-hidden="true" /> }
            />
        }
    };
}

#[component(transparent)]
pub fn LabRoute() -> impl IntoView {
    #[cfg(feature = "lab")]
    {
        view! {
            <Route path=ROUTE_LAB view=move || view! {
                <RootLayout><LabPage /></RootLayout>
            } />
        }
    }
    #[cfg(not(feature = "lab"))]
    {
        noop_route!("lab")
    }
}

#[component(transparent)]
pub fn TidyTuesdayEntryRoute() -> impl IntoView {
    #[cfg(feature = "tidy-tuesday")]
    {
        view! {
            <Route path=ROUTE_TIDY_TUESDAY_ENTRY view=move || view! {
                <RootLayout><TidyTuesdayPage /></RootLayout>
            } />
        }
    }
    #[cfg(not(feature = "tidy-tuesday"))]
    {
        noop_route!("tidy-tuesday-entry")
    }
}

#[component(transparent)]
pub fn TidyTuesdayIndexRoute() -> impl IntoView {
    #[cfg(feature = "tidy-tuesday")]
    {
        view! {
            <Route path=ROUTE_TIDY_TUESDAY view=move || view! {
                <RootLayout><TidyTuesdayIndexPage /></RootLayout>
            } />
        }
    }
    #[cfg(not(feature = "tidy-tuesday"))]
    {
        noop_route!("tidy-tuesday-index")
    }
}

#[component(transparent)]
pub fn SituationMonitorRoute() -> impl IntoView {
    #[cfg(feature = "situation-monitor")]
    {
        view! {
            <Route path=ROUTE_SITUATION_MONITOR view=move || view! {
                <RootLayout><SituationMonitorPage /></RootLayout>
            } />
        }
    }
    #[cfg(not(feature = "situation-monitor"))]
    {
        noop_route!("situation-monitor")
    }
}

#[component(transparent)]
pub fn MalwareTrafficEntryRoute() -> impl IntoView {
    #[cfg(feature = "malware-traffic")]
    {
        view! {
            <Route path=ROUTE_MALWARE_REPORTS_ENTRY view=move || view! {
                <RootLayout><MalwareTrafficPage /></RootLayout>
            } />
        }
    }
    #[cfg(not(feature = "malware-traffic"))]
    {
        noop_route!("malware-traffic-entry")
    }
}

#[component(transparent)]
pub fn MalwareTrafficIndexRoute() -> impl IntoView {
    #[cfg(feature = "malware-traffic")]
    {
        view! {
            <Route path=ROUTE_MALWARE_REPORTS view=move || view! {
                <RootLayout><MalwareTrafficIndexPage /></RootLayout>
            } />
        }
    }
    #[cfg(not(feature = "malware-traffic"))]
    {
        noop_route!("malware-traffic-index")
    }
}
