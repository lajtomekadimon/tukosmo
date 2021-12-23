
let site_languages = document.getElementById("site-languages");

document.getElementById("button-select-language").addEventListener(
    'click',
    function() {
        site_languages.style.display = "block";
        site_languages.style.opacity = '1';
    }
);

document.getElementById("close-site-languages").addEventListener(
    'click',
    function() {
        site_languages.style.opacity = '0';
    }
);

document.getElementById("site-languages-bg").addEventListener(
    'click',
    function() {
        site_languages.style.opacity = '0';
    }
);

site_languages.addEventListener(
    'transitionend',
    function() {
        if (site_languages.style.display === "block") {
            site_languages.style.display = "none";
        }
    }
);

