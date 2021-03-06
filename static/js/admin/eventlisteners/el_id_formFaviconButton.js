
function el_id_formFaviconButton() {
    const elem = getId("form-favicon-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                elem.classList.add("is-loading");
                getId("form-favicon-progress").classList.remove("is-hidden");

                const nexturl = elem.dataset.nexturl;

                const form = getId("form-favicon");
                const form_url = form.action;

                function check_website_is_online() {
                    sleep(1000);

                    fetch("/").then(
                        response => {
                            window.location.href = nexturl;
                        }
                    ).then(
                        response => {
                            window.location.href = nexturl;
                        }
                    ).catch(
                        error => {
                            check_website_is_online();
                        }
                    );
                }

                fetch(
                    form_url,
                    {
                        method: "POST",
                        body: new FormData(form)
                    }
                ).then(
                    response => {
                        check_website_is_online();
                    }
                ).catch(
                    error => {
                        check_website_is_online();
                    }
                );
            }
        );
    }
}

el_id_formFaviconButton();
