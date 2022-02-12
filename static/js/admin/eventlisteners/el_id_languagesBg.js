
function el_id_languagesBg() {
    const elem = getId("languages-bg");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const languages = getId("languages");
                languages.classList.remove("is-active");
            }
        );
    }
}

el_id_languagesBg();
