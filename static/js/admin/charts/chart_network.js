
function chart_network() {
    const ctx = getId('chart-network');

    if (ctx !== null) {
        const ctx_data = getId('chart-network-data');
        const title_value = ctx_data.dataset.title;
        var labels_array = ctx_data.dataset.labels.split(",");
        const lupload_value = ctx_data.dataset.lupload;
        const uploaded_array = ctx_data.dataset.uploaded.split(",");
        const ldownload_value = ctx_data.dataset.ldownload;
        const downloaded_array = ctx_data.dataset.downloaded.split(",");

        var labels_array = labels_array.map(
            function(e) {
                return e.slice(0, -6);
            }
        )

        const data = {
            labels: labels_array,
            datasets: [{
                label: lupload_value,
                data: uploaded_array,
                fill: false,
                borderColor: '#ef280c',
                tension: 0.1
            }, {
                label: ldownload_value,
                data: downloaded_array,
                fill: false,
                borderColor: '#3884b7',
                tension: 0.1
            }]
        };

        const config = {
            type: 'line',
            data: data,
            options: {
                responsive: false,
                plugins: {
                    legend: {
                        position: 'top',
                    },
                    title: {
                        display: true,
                        text: title_value
                    }
                }
            }
        };

        const myChart = new Chart(ctx, config);
    }
}

chart_network();
