
const id_dropdownLangButton = document.getElementById("dropdown-lang-button");

if (id_dropdownLangButton !== null) {
    id_dropdownLangButton.addEventListener(
        'click',
        function() {
            const id_dropdownLang = document.getElementById("dropdown-lang");
            id_dropdownLang.classList.toggle('is-active');
        }
    );
}

