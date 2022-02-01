
function el_id_formWebsiteButton() {
    const elem = getId("form-website-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                getId("form-website-progress").classList.remove("is-hidden");

                const nexturl = elem.dataset.nexturl;

                const form = getId("form-website");
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

                        check_website_is_online();
                    }
                );
            }
        );
    }
}

el_id_formWebsiteButton();
