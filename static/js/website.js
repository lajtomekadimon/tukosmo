
let languages = document.getElementById("languages");

document.getElementById("button-select-language").addEventListener(
    'click',
    function() {
        languages.style.display = "block";
        languages.style.opacity = "1";
    }
);

document.getElementById("languages-close").addEventListener(
    'click',
    function() {
        languages.style.opacity = "0";
    }
);

document.getElementById("languages-bg").addEventListener(
    'click',
    function() {
        languages.style.opacity = "0";
    }
);

languages.addEventListener(
    'transitionend',
    function() {
        if (languages.style.display === "block") {
            languages.style.display = "none";
        }
    }
);

document.getElementById("navigation-burger").addEventListener(
    'click',
    function() {
        let navigation_ul = document.getElementById("navigation-ul");

        if (navigation_ul.style.display === "block") {
            navigation_ul.style.display = "none";
        } else {
            navigation_ul.style.display = "block";
        }
    }
);
