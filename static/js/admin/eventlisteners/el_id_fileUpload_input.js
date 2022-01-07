
function el_id_fileUpload_input() {
    const elem = document.querySelector('#file-upload input[type=file]');

    if (elem !== null) {
        elem.onchange = () => {
            if (elem.files.length > 0) {
                const class_fileName =
                    document.querySelector('#file-upload .file-name');
                class_fileName.textContent = elem.files[0].name;

                // Show progress bar
                const id_fileUploadProgress = getId("file-upload-progress");
                id_fileUploadProgress.style.display = "block";

                // Upload file
                const id_fileUploadFile = getId("file-upload-file");
                const post_url = id_fileUploadFile.dataset.url;
                const files = id_fileUploadFile.files;
                const formData = new FormData();
                formData.append('myFile', files[0]);
                fetch(
                    post_url,
                    {
                        method: 'POST',
                        body: formData
                    }
                ).then(
                    response => response.json()
                ).then(
                    json_resp => {
                        if (json_resp.success) {
                            const id_fileUploadForm =
                                getId("file-upload-form");

                            // Fill id and filename
                            const id_fileUploadForm_id =
                                id_fileUploadForm.querySelector(
                                    '#file-upload-form input[name=id]'
                                );
                            id_fileUploadForm_id.value = json_resp.id;
                            const id_fileUploadForm_filename =
                                id_fileUploadForm.querySelector(
                                    '#file-upload-form input[name=filename]'
                                );
                            id_fileUploadForm_filename.value =
                                json_resp.filename;

                            // Hide progress bar
                            id_fileUploadProgress.style.display = "none";

                            // Show success notification
                            const id_fileUploadNotifSuccess =
                                getId("file-upload-notif-success");
                            id_fileUploadNotifSuccess.style.display = "block";

                            // Show form
                            id_fileUploadForm.style.display = "block";
                        } else {
                            console.error("Something went bad.");
                            // TODO: Show error, etc.
                        }
                    }
                ).catch(
                    error => {
                        // TODO: Show error, etc.
                        console.error(error);
                    }
                );
            }
        };
    }
}

el_id_fileUpload_input();
