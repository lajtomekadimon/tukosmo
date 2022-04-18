
function chart_memory() {
    const ctx = getId('chart-memory');

    if (ctx !== null) {
        const ctx_data = getId('chart-memory-data');
        const title_value = ctx_data.dataset.title;
        var labels_array = ctx_data.dataset.labels.split(",");
        const lmemory_value = ctx_data.dataset.lmemory;
        const memory_array = ctx_data.dataset.memory.split(",");

        labels_array = labels_array.map(
            function(e) {
                return e.slice(0, -6);
            }
        )

        const data = {
            labels: labels_array,
            datasets: [{
                label: lmemory_value,
                data: memory_array,
                fill: false,
                borderColor: '#af248f',
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

chart_memory();
