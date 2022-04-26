use markup;


markup::define! {
    Icon<'a>(
        name: &'a str,
    ) {
        i[
            class = "eos-icons notranslate",
            translate = "no",
        ] {
            @name
        }
    }

    IconSIZE5<'a>(
        name: &'a str,
    ) {
        i[
            class = "eos-icons notranslate is-size-6",
            translate = "no",
        ] {
            @name
        }
    }

    IconSIZE6<'a>(
        name: &'a str,
    ) {
        i[
            class = "eos-icons notranslate is-size-6",
            translate = "no",
        ] {
            @name
        }
    }

    /*----------------*/

    Article {
        @Icon { name: "article" }
    }

    BarChart {
        @Icon { name: "bar_chart" }
    }

    CalendarToday {
        @Icon { name: "calendar_today" }
    }

    Check {
        @Icon { name: "check" }
    }

    CloudUpload {
        @Icon { name: "cloud_upload" }
    }

    Dashboard {
        @Icon { name: "dashboard" }
    }

    Database {
        @Icon { name: "database" }
    }

    Description {
        @Icon { name: "description" }
    }

    Devices {
        @Icon { name: "devices" }
    }

    DNS {
        @Icon { name: "dns" }
    }

    FilePresent {
        @Icon { name: "file_present" }
    }

    HomeSIZE6 {
        @IconSIZE6 { name: "home" }
    }

    KeyboardArrowDown {
        @Icon { name: "keyboard_arrow_down" }
    }

    Label {
        @Icon { name: "label" }
    }

    Language {
        @Icon { name: "language" }
    }

    LanguageSIZE6 {
        @IconSIZE6 { name: "language" }
    }

    Link {
        @Icon { name: "link" }
    }

    List {
        @Icon { name: "list" }
    }

    Login {
        @Icon { name: "login" }
    }

    LogoutSIZE5 {
        @IconSIZE5 { name: "logout" }
    }

    Menu {
        @Icon { name: "menu" }
    }

    Memory {
        @Icon { name: "memory" }
    }

    ModeEdit {
        @Icon { name: "mode_edit" }
    }

    OpenInBrowser {
        @Icon { name: "open_in_browser" }
    }

    OpenInNew {
        @Icon { name: "open_in_new" }
    }

    Payments {
        @Icon { name: "payments" }
    }

    Person {
        @Icon { name: "person" }
    }

    Photo {
        @Icon { name: "photo" }
    }

    PhotoLibrary {
        @Icon { name: "photo_library" }
    }

    ProductSubscriptions {
        @Icon { name: "product_subscriptions" }
    }

    Quiz {
        @Icon { name: "quiz" }
    }

    Search {
        @Icon { name: "search" }
    }

    Sell {
        @Icon { name: "sell" }
    }

    SettingsSuggest {
        @Icon { name: "settings_suggest" }
    }

    Store {
        @Icon { name: "store" }
    }

    SupervisorAccount {
        @Icon { name: "supervisor_account" }
    }

    SyncDisabled {
        @Icon { name: "sync_disabled" }
    }

    SyncI {
        @Icon { name: "sync" }
    }

    Translate {
        @Icon { name: "translate" }
    }

    ViewSidebar {
        @Icon { name: "view_sidebar" }
    }

    VPNKey {
        @Icon { name: "vpn_key" }
    }

    Web {
        @Icon { name: "web" }
    }
}

