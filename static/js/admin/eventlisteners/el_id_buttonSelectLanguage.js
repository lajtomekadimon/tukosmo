
function el_id_buttonSelectLanguage() {
    const elem = getId("button-select-language");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const languages = getId("languages");
                languages.classList.add("is-active");
            }
        );
    }
}

el_id_buttonSelectLanguage();
