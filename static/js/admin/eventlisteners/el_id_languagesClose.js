
function el_id_languagesClose() {
    const elem = getId("languages-close");

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

el_id_languagesClose();
