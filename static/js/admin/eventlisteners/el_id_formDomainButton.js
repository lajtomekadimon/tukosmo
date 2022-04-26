
function el_id_formDomainButton() {
    const elem = getId("form-domain-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                elem.classList.add("is-loading");
                getId("form-domain-progress").classList.remove("is-hidden");

                const newurl =
                    "https://".concat(getId("form-domain-domain").value);

                const form = getId("form-domain");
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
                        body: new URLSearchParams(
                            new FormData(form)
                        )
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

el_id_formDomainButton();
