pub struct SidenavItem {
    pub text: &'static str,
    pub href: &'static str,
}

pub const SIDENAV_ITEMS: &[SidenavItem] = &[
    SidenavItem {
        text: "Kanban",
        href: "/",
    },
    SidenavItem {
        text: "Backlog",
        href: "/backlog",
    },
    SidenavItem {
        text: "Boards",
        href: "/boards",
    },
];
