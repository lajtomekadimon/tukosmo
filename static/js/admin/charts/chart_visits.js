
function chart_visits() {
    const ctx = getId('chart-visits');

    if (ctx !== null) {
        const ctx_data = getId('chart-visits-data');
        const title_value = ctx_data.dataset.title;
        var labels_array = ctx_data.dataset.labels.split(",");
        const lvisitors_value = ctx_data.dataset.lvisitors;
        const visitors_array = ctx_data.dataset.visitors.split(",");
        const lvisits_value = ctx_data.dataset.lvisits;
        const visits_array = ctx_data.dataset.visits.split(",");

        labels_array = labels_array.map(
            function(e) {
                return e.slice(0, -6);
            }
        )

        const data = {
            labels: labels_array,
            datasets: [{
                label: lvisitors_value,
                data: visitors_array,
                fill: {
                    target: 'origin',
                    //below: '#7b4fbc'
                },
                borderColor: '#7b4fbc',
                tension: 0.1
            }, {
                label: lvisits_value,
                data: visits_array,
                fill: false,
                borderColor: '#16a0fc',
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
                        ticks: {
                            stepSize: 1
                        },
                        beginAtZero: true
                    }
                },
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

chart_visits();
