
function el_id_ttagAddButton() {
    const elem = getId("ttag-add-button");

    if (elem !== null) {
        elem.addEventListener(
            'click',
            function() {
                const id_ttagsList = getId("ttags-list");
                const tag_id = id_ttagsList.value;
                const tag_name =
                    id_ttagsList.options[id_ttagsList.selectedIndex].text;

                const maybe_tagInput = getId("tag-input-" + tag_id);
                if (maybe_tagInput == null) {
                    const html_input = document.createElement("input");
                    html_input.setAttribute("id", "tag-input-" + tag_id);
                    html_input.setAttribute("type", "hidden");
                    html_input.setAttribute("name", "tag");
                    html_input.setAttribute("value", tag_id);

                    const html_label = document.createElement("div");
                    html_label.setAttribute("id", "tag-label-" + tag_id);
                    html_label.setAttribute("class", "control");
                    const html_l = document.createElement("div");
                    html_l.setAttribute("class", "tags has-addons");
                    const html_l_a1 = document.createElement("a");
                    html_l_a1.setAttribute("class", "tag is-link");
                    html_l_a1.textContent = tag_name;
                    const html_l_a2 = document.createElement("a");
                    html_l_a2.setAttribute(
                        "class",
                        "tag is-delete ttag-remove"
                    );
                    html_l_a2.setAttribute("data-id", tag_id);
                    html_l.appendChild(html_l_a1);
                    html_l.appendChild(html_l_a2);
                    html_label.appendChild(html_l);

                    const id_ttagsAdded = getId("ttags-added");
                    id_ttagsAdded.appendChild(html_input);
                    id_ttagsAdded.appendChild(html_label);

                    // Load again
                    el_classes_ttagRemove();
                }
            }
        );
    }
}

el_id_ttagAddButton();
