
let button_select_language = document.getElementById("button-select-language");

button_select_language.addEventListener('click', function() {
    document.getElementById("site-languages").style.display = "block";
});

let site_languages_bg = document.getElementById("site-languages-bg");

site_languages_bg.addEventListener('click', function() {
    document.getElementById("site-languages").style.display = "none";
});
