enum Locale {
    Hungarian,
    English,
    Slovak,
}

struct ApplicationLocale {}

struct MenubarLocale {
    menubar_lang_button: String,
}

struct OptionsWindowLocale {
    options_window_title: String,
    n_o_point_label: String,
    n_o_nodes_label: String,
    stepsize_label: String,
    generate_button: String,
    start_button: String,
    stop_button: String,
    next_gen_button: String,
    reset_button: String,
}

struct GraphWindowLocale {
    author_label: String,
    solution_label: String,
    temperature_label: String,
}
