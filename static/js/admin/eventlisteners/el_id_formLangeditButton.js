
function el_id_formLangeditButton() {
    const elem = getId("form-langedit-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                getId("form-langedit-progress").classList.remove("is-hidden");

                const nexturl = elem.dataset.nexturl;

                const form = getId("form-langedit");
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

el_id_formLangeditButton();
