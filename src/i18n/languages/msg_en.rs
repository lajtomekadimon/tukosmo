use crate::i18n::translate_i18n::TranslateI18N;


pub const MSG_EN: TranslateI18N = TranslateI18N {
    account:
        "Account",
    account_status:
        "Account status",
    active_k_account:
        "Active",
    add_k_verb:
        "Add",
    add_language:
        "Add language",
    admin:
        "Admin",
    all_data_created_by_user_will_be_deleted_w_user:
        "All data created by {user} will be PERMANENTLY deleted.",
    appearance:
        "Appearance",
    are_you_sure_that_you_want_to_delete_this_file:
        "Are you sure that you want to delete this file?",
    are_you_sure_that_you_want_to_delete_this_language:
        "Are you sure that you want to delete this language?",
    are_you_sure_that_you_want_to_delete_this_post:
        "Are you sure that you want to delete this post?",
    are_you_sure_that_you_want_to_delete_this_tag:
        "Are you sure that you want to delete this tag?",
    are_you_sure_that_you_want_to_delete_this_user:
        "Are you sure that you want to delete this user?",
    author:
        "Author",
    autocomplete_for:
        "Autocomplete for:",
    blog:
        "Blog",
    browser:
        "Browser",
    browsers_k_web:
        "Browsers",
    cancel:
        "Cancel",
    categories:
        "Categories",
    change_filename_question:
        "Change file name?",
    choose_a_file:
        "Choose a file...",
    close_k_session:
        "Close",
    close_k_verb:
        "Close",
    code:
        "Code",
    copyright_owner:
        "Copyright owner",
    copyright_owner_in_the_new_language:
        "Copyright owner (in the new language)",
    copyright_w_year_name:
        "© {year} {name}",
    core_k_cpu:
        "core",
    countries:
        "Countries",
    cpu_usage_percent:
        "CPU usage (%)",
    current_favicon:
        "Current favicon",
    current_password:
        "Current password",
    dashboard:
        "Dashboard",
    data:
        "Data",
    date:
        "Date",
    default_language:
        "Default language",
    delete:
        "Delete",
    delete_file:
        "Delete file",
    delete_file_w_name:
        "Delete file: {name}",
    delete_language_w_name:
        "Delete language: {name}",
    delete_permanent:
        "Delete (permanent)",
    delete_post_w_title:
        "Delete post: '{title}'",
    delete_tag_w_name:
        "Delete tag: {name}",
    delete_user_w_name:
        "Delete user: {name}",
    deleted_k_post:
        "Deleted",
    deleted_posts:
        "Deleted posts",
    description:
        "Description",
    device:
        "Device",
    disk_size_w_used_free_total_u1_u2_u3:
        "{used} {u1} of {total} {u3} ({free} {u2} free)",
    disk_usage:
        "Disk usage",
    disk_usage_w_unit:
        "Disk usage ({unit})",
    documentation:
        "Documentation",
    domain_cant_be_changed_in_development_mode:
        "Domain can't be changed in development mode.",
    domain_k_web:
        "Domain",
    download:
        "Download",
    download_k_noun:
        "Download",
    draft:
        "Draft",
    draft_posts:
        "Draft posts",
    drafts:
        "Drafts",
    edit:
        "Edit",
    edit_file:
        "Edit file",
    edit_file_w_name:
        "Edit file: {name}",
    edit_language_w_name:
        "Edit language: {name}",
    edit_post_w_title:
        "Edit post: '{title}'",
    edit_tag_w_name:
        "Edit tag: {name}",
    edit_user_w_name:
        "Edit user: {name}",
    email:
        "Email",
    err_csrf_token_is_not_a_valid_uuid:
        "The CSRF token is not a valid UUID.",
    err_email_already_exists:
        "That email is already in use.",
    err_default_language_cant_be_deleted:
        "The default language can't be deleted; \
        choose another default language first.",
    err_featured_image_is_not_image:
        "The featured image file must be an image.",
    err_field_is_not_lang_code:
        "The language code is not valid.",
    err_file_title_already_exists:
        "That file title is already in use.",
    err_filename_already_exists:
        "That file name is already in use.",
    err_lang_code_already_exists:
        "That language code already exists.",
    err_passwords_do_not_match:
        "Passwords do not match.",
    err_some_wrong_i18n_user_name:
        "Some of the user names for each language are not valid.",
    err_some_wrong_lang_id_of_name:
        "Some of the language IDs are not correct.",
    err_some_wrong_lang_name:
        "Some of the language names are not valid.",
    err_some_wrong_name_for_lang:
        "Some of the names (in the new language) are not valid.",
    err_some_wrong_tag_name:
        "Some of the tag names are not valid.",
    err_some_wrong_tag_permalink:
        "Some of the tag permalinks are not valid.",
    err_suspended_account:
        "That user account is suspended.",
    err_unknown_error:
        "Unknown error.",
    err_user_cant_delete_itself:
        "A user can't delete itself.",
    err_user_is_not_admin:
        "This user doesn't have admin permissions.",
    err_user_not_logged_in:
        "You must be logged in.",
    err_uuid_is_not_valid:
        "The UUID is not valid.",
    err_wrong_body_text:
        "Body text can't be empty.",
    err_wrong_copyright_owner:
        "The copyright owner is not valid.",
    err_wrong_csrf_token:
        "The CSRF token is not correct.",
    err_wrong_description:
        "Description can't start or begin with empty space; \
        it can't have more than 600 characters either.",
    err_wrong_domain:
        "That domain is not valid.",
    err_wrong_email:
        "That email is not valid.",
    err_wrong_file_id:
        "That file ID is not correct.",
    err_wrong_file_title:
        "That file title is not valid.",
    err_wrong_filename:
        "That file name is not valid. Only alphanumeric (A-Z, a-z, 0-9),
         dot (.), hyphen (-) and underscore (_) characters are allowed.",
    err_wrong_lang_code:
        "The language code is not correct.",
    err_wrong_lang_id:
        "The language ID is not correct.",
    err_wrong_own_lang_name:
        "The language name (in the new language) is not valid.",
    err_wrong_page_number:
        "The page number is not correct.",
    err_wrong_password: 
        "That password is not valid; it should have 8 or more characters.",
    err_wrong_permalink:
        "The permalink is not valid.",
    err_wrong_post_id:
        "The post ID is not correct.",
    err_wrong_post_permalink:
        "That post doesn't exist.",
    err_wrong_rpp_number:
        "The number of results per page is not correct.",
    err_wrong_tag_id:
        "The tag ID is not correct.",
    err_wrong_title:
        "The title is not valid.",
    err_wrong_user_email:
        "No user has that email.",
    err_wrong_user_id:
        "The user ID is not correct.",
    err_wrong_user_name:
        "That user name is not valid.",
    err_wrong_user_password:
        "The password is not correct.",
    err_wrong_website_subtitle:
        "The website's subtitle is not valid.",
    err_wrong_website_title:
        "The website's title is not valid.",
    error:
        "Error",
    examples_of_lang_codes:
        "Examples: en, en-us...",
    examples_of_lang_names:
        "Examples: English, Español...",
    external_referrals:
        "External referrals",
    favicon:
        "Favicon",
    favicon_note:
        "NOTE: If your web browser doesn't load the new favicon, \
         you can force a reload with Ctrl+Shift+R.",
    featured_image:
        "Featured image",
    file:
        "File",
    file_title:
        "File title",
    filename:
        "File name",
    files:
        "Files",
    forgotten_password:
        "Forgotten password?",
    forgotten_password_message_info:
        "Enter your email address and you will receive an email message
         with a link to reset your password.",
    free_k_disk:
        "Free",
    general:
        "General",
    go_back_to_w_website:
        "Go back to {website}",
    hello_user:
        "Hello, {name}.",
    help:
        "Help",
    i_hope_you_are_having_a_great_day:
        "I hope you are having a great day!",
    lang_code:
        "en",
    lang_en:
        "English",
    lang_es:
        "Spanish",
    lang_ja:
        "Japanese",
    language:
        "Language",
    language_name:
        "Language name",
    language_names:
        "Language names",
    languages:
        "Languages",
    last_30_days_month:
        "Last 30 days (month)",
    last_365_days_year:
        "Last 365 days (year)",
    last_7_days_week:
        "Last 7 days (week)",
    last_start_w_date:
        "Last start: {date}",
    last_update:
        "Last update",
    login_k_noun:
        "Login",
    login_k_verb:
        "Login",
    logout_k_verb:
        "Logout",
    memory_k_ram:
        "Memory",
    memory_usage_percent:
        "Memory usage (%)",
    menu:
        "Menu",
    menus:
        "Menus",
    n_days_w_time:
        "{time} days",
    n_hours_w_time:
        "{time} hours",
    n_minutes_w_time:
        "{time} minutes",
    n_months_w_time:
        "{time} months",
    n_posts:
        "{n} posts",
    n_results_of_m:
        "{n} results of {m}",
    n_seconds_w_time:
        "{time} seconds",
    n_years_w_time:
        "{time} years",
    name:
        "Name",
    name_in_the_new_language:
        "Name (in the new language)",
    names_for_each_language:
        "Names for each language",
    names_in_the_new_language_for_each_language:
        "Names (in the new language) for each language",
    network_w_unit:
        "Network ({unit})",
    new_category:
        "New category",
    new_domain_k_web:
        "New domain",
    new_password:
        "New password",
    new_post:
        "New post",
    new_tag:
        "New tag",
    new_user:
        "New user",
    next:
        "Next",
    no_file_uploaded:
        "No file uploaded",
    number_of_visits:
        "Number of visits",
    one_day:
        "1 day",
    one_hour:
        "1 hour",
    one_minute:
        "1 minute",
    one_month:
        "1 month",
    one_result_of_m:
        "1 result of {m}",
    one_second:
        "1 second",
    one_year:
        "1 year",
    open_in_new_tab:
        "Open in new tab",
    optional:
        "optional",
    original_author:
        "Original author",
    page_n:
        "Page {n}",
    pages:
        "Pages",
    password:
        "Password",
    permalink:
        "Permalink",
    platforms_k_os:
        "Platforms",
    please_visit_new_domain_w_domain:
        "Please visit https://{domain}",
    png_image:
        "PNG image",
    posts:
        "Posts",
    post_s_body:
        "Post's body",
    previous:
        "Previous",
    published:
        "Published",
    published_k_posts:
        "Published",
    published_posts:
        "Published posts",
    read_more:
        "Read more",
    remove_featured_image:
        "Remove featured image",
    repeat_new_password:
        "Repeat new password",
    repeat_password:
        "Repeat password",
    reset_password:
        "Reset password",
    reset_password_success_w_email:
        "We have sent an email to {email} with a link to \
         reset your password. It expires in 20 minutes.",
    same_domain_recieved_nothing_changed:
        "Some domain recieved; nothing changed.",
    scheduled_k_posts:
        "Scheduled",
    see_languages:
        "See languages",
    select_a_language:
        "Select a language",
    select_a_tag:
        "Select a tag",
    server:
        "Server",
    sessions:
        "Sessions",
    settings:
        "Settings",
    show_more:
        "Show more",
    sign_up_k_verb:
        "Sign up",
    since:
        "Since",
    size_k_file:
        "Size",
    statistics:
        "Statistics",
    status:
        "Status",
    submit:
        "Submit",
    suspended_account:
        "Suspended account",
    suspended_k_account:
        "Suspended",
    swap_k_memory:
        "Swap",
    system_k_os:
        "System",
    tags:
        "Tags",
    the_post_will_be_permanently_deleted:
        "The post will be permanent deleted.",
    the_post_will_be_sent_to_trash:
        "The post will be sent to trash.",
    theme:
        "Theme",
    there_are_languages_without_names:
        "There are languages without names.",
    title:
        "Title",
    today:
        "Today",
    total:
        "Total",
    translated_by_user:
        "translated by {name}",
    trash:
        "Trash",
    tukosmo:
        "Tukosmo",
    tukosmo_admin_panel:
        "Tukosmo Admin Panel",
    tukosmo_has_been_running_for_w_duration:
        "Tukosmo has been running for {duration} straight.",
    untranslated:
        "Untranslated",
    untranslated_k_lower:
        "untranslated",
    untranslated_k_posts:
        "Untranslated",
    untranslated_posts:
        "Untranslated posts",
    update:
        "Update",
    upload_file:
        "Upload file",
    upload_k_noun:
        "Upload",
    upload_new_favicon:
        "Upload new favicon",
    uploaded_by:
        "Uploaded by",
    uploaded_by_name_on_date:
        "Uploaded by {name} ({date})",
    uploaded_on:
        "Uploaded on",
    used_k_disk:
        "Used",
    users:
        "Users",
    visit_website:
        "Visit website",
    visitors:
        "Visitors",
    visits:
        "Visits",
    warning_domain_paragraph:
        "WARNING: Before doing anything, you must add a record of type A \
        pointing to your server's IP and a record of type CAA \
        (128 issue \"letsencrypt.org\") in the DNS zone of your new domain; \
        sometimes you have to wait hours or days until that change is \
        applied. If you don't do this, Tukosmo might stop working and you'll \
        need manual intervention on your server. Be careful!",
    web_browsers_preview:
        "Web browsers preview",
    website:
        "Website",
    website_subtitle:
        "Website subtitle",
    website_subtitle_in_the_new_language:
        "Website subtitle (in the new language)",
    website_title:
        "Website title",
    website_title_in_the_new_language:
        "Website title (in the new language)",
    widgets:
        "Widgets",
    yesterday:
        "Yesterday",
    youll_receive_any_expiry_notices_of_tls_certificates:
        "You'll receive any expiry notices of TLS certificates \
        (only if Tukosmo wasn't able to renew them automatically).",
    your_account_has_been_successfully_updated:
        "Your account has been successfully updated.",
    your_current_domain_is_w_domain:
        "Your current domain is: {domain}",
    your_email:
        "Your email",
    your_file_has_been_successfully_uploaded:
        "Your file has been uploaded.",
    your_password:
        "Your password",
    your_sessions_have_been_successfully_updated:
        "Your sessions have been successfully updated.",
    your_website_categories_were_successfully_updated:
        "Your website categories were successfully updated.",
    your_website_favicon_was_successfully_updated:
        "Your website favicon was successfully updated.",
    your_website_files_were_successfully_updated:
        "Your website files were successfully updated.",
    your_website_info_has_been_successfully_updated:
        "Your website information has been successfully updated.",
    your_website_languages_were_successfully_updated:
        "Your website languages were successfully updated.",
    your_website_posts_were_successfully_updated:
        "Your website posts were successfully updated.",
    your_website_tags_were_successfully_updated:
        "Your website tags were successfully updated.",
    your_website_users_were_successfully_updated:
        "Your website users were successfully updated.",
};
