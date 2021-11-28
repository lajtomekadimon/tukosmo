
let dropdown_lang_button = document.getElementById("dropdown-lang-button");

if (dropdown_lang_button !== null) {
    dropdown_lang_button.addEventListener('click', function() {
        document.getElementById("dropdown-lang").classList.toggle('is-active');
    });
}


let navbar_burger_button = document.getElementById("navbar-burger");

if (navbar_burger_button !== null) {
    navbar_burger_button.addEventListener('click', function() {
        document.getElementById("navMenu").classList.toggle('is-active');
    });
}

