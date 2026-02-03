use crate::components::{ChartCard, StatsCard, TableCard};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
#[allow(non_snake_case)]
pub fn AdminDashboardPage() -> impl IntoView {
    view! {
        <Title text="Admin Dashboard"/>
        <div class="space-y-8">
            // Stats Grid
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <StatsCard
                    title="Total Subscribers"
                    value="238.4M"
                    change="+8.5%"
                    trend="up"
                    icon="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                />
                <StatsCard
                    title="Monthly Revenue"
                    value="$8.54B"
                    change="+12.3%"
                    trend="up"
                    icon="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
                <StatsCard
                    title="Watch Time"
                    value="1.2B hrs"
                    change="+18.7%"
                    trend="up"
                    icon="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                />
                <StatsCard
                    title="Content Library"
                    value="15,847"
                    change="+234"
                    trend="up"
                    icon="M7 4v16M17 4v16M3 8h4m10 0h4M3 12h18M3 16h4m10 0h4M4 20h16a1 1 0 001-1V5a1 1 0 00-1-1H4a1 1 0 00-1 1v14a1 1 0 001 1z"
                />
            </div>

            // Charts Row
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <ChartCard title="Revenue Overview" />
                <ChartCard title="User Growth" />
            </div>

            // Tables
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <TableCard title="Top Content" />
                <TableCard title="Recent Activities" />
            </div>
        </div>
    }
}
