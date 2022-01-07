
function el_id_dropdownLangButton() {
    const elem = getId("dropdown-lang-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const id_dropdownLang = document.getId("dropdown-lang");
                id_dropdownLang.classList.toggle('is-active');
            }
        );
    }
}

el_id_dropdownLangButton();
