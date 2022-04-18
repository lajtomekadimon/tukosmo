
function chart_cpu() {
    const ctx = getId('chart-cpu');

    if (ctx !== null) {
        const ctx_data = getId('chart-cpu-data');
        const title_value = ctx_data.dataset.title;
        var labels_array = ctx_data.dataset.labels.split(",");
        const lcore_value = ctx_data.dataset.lcore;
        const cores_array = ctx_data.dataset.cores.split(",");

        labels_array = labels_array.map(
            function(e) {
                return e.slice(0, -6);
            }
        )

        const data = {
            labels: labels_array,
            datasets: [{
                label: lcore_value,
                data: cores_array,
                fill: false,
                borderColor: '#e6194b',
                tension: 0.1
            }]
        };

        const config = {
            type: 'line',
            data: data,
            options: {
                responsive: false,
                scales: {
                    y: {
                        beginAtZero: true,
                        max: 100
                    }
                },
                plugins: {
                    legend: {
                        display: false,
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

chart_cpu();
