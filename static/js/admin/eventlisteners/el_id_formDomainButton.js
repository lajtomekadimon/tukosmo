
function el_id_formDomainButton() {
    const elem = getId("form-domain-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                getId("form-domain-progress").classList.remove("is-hidden");

                const newurl =
                    "https://".concat(getId("form-domain-domain").value);

                const form = getId("form-domain");
                const form_url = form.action;

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
                        form.submit();
                    }
                ).then(
                    response => {
                        form.submit();
                    }
                ).catch(
                    error => {
                        function check_website_is_online() {
                            sleep(5000);

                            fetch(newurl).then(
                                response => {
                                    window.location.href = newurl;
                                }
                            ).then(
                                response => {
                                    window.location.href = newurl;
                                }
                            ).catch(
                                error => {
                                    check_website_is_online();
                                }
                            );
                        }

                        check_website_is_online();
                    }
                );
            }
        );
    }
}

el_id_formDomainButton();
