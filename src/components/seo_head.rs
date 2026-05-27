use leptos::*;
use leptos_meta::*;
use leptos_router::use_location;

use crate::seo::{seo_for_path, DEFAULT_DESCRIPTION, DEFAULT_TITLE, OG_IMAGE_URL, SITE_NAME};

#[component]
pub fn SeoHead() -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();

    let title = create_rw_signal(DEFAULT_TITLE.to_string());
    let description = create_rw_signal(DEFAULT_DESCRIPTION.to_string());
    let canonical = create_rw_signal(String::new());

    create_effect(move |_| {
        let seo = seo_for_path(&pathname());
        let canonical_url = seo.canonical_url();
        title.set(seo.title);
        description.set(seo.description);
        canonical.set(canonical_url);
    });

    view! {
        <Title text=move || title.get() />
        <Link rel="canonical" attr:href=move || canonical.get() />
        <Meta name="description" content=move || description.get() />
        <Meta name="author" content=SITE_NAME />
        <Meta name="robots" content="index, follow" />
        <Meta property="og:type" content="website" />
        <Meta property="og:site_name" content=SITE_NAME />
        <Meta property="og:title" content=move || title.get() />
        <Meta property="og:description" content=move || description.get() />
        <Meta property="og:url" content=move || canonical.get() />
        <Meta property="og:image" content=OG_IMAGE_URL />
        <Meta property="og:image:width" content="1200" />
        <Meta property="og:image:height" content="630" />
        <Meta property="og:image:alt" content="Jack Weekly portfolio: strategic and technical analysis." />
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:title" content=move || title.get() />
        <Meta name="twitter:description" content=move || description.get() />
        <Meta name="twitter:image" content=OG_IMAGE_URL />
    }
}
