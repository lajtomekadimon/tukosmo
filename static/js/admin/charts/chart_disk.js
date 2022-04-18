
function chart_disk() {
    const ctx = getId('chart-disk');

    if (ctx !== null) {
        const ctx_data = getId('chart-disk-data');
        const title_value = ctx_data.dataset.title;
        const lused_value = ctx_data.dataset.lused;
        const used_value = ctx_data.dataset.used;
        const lfree_value = ctx_data.dataset.lfree;
        const free_value = ctx_data.dataset.free;

        const data = {
            labels: [lused_value, lfree_value],
            datasets: [{
                label: 'GiB',
                data: [used_value, free_value],
                backgroundColor: ["#3e95cd", "#d8d8d8"]
            }]
        };

        const config = {
            type: 'pie',
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

chart_disk();
