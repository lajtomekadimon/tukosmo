
function chart_referrals() {
    const ctx = getId('chart-referrals');

    if (ctx !== null) {
        const ctx_data = getId('chart-referrals-data');
        const title_value = ctx_data.dataset.title;
        const labels_array = ctx_data.dataset.labels.split(",");
        const lvisitors_value = ctx_data.dataset.lvisitors;
        const visitors_array = ctx_data.dataset.visitors.split(",");
        const lvisits_value = ctx_data.dataset.lvisits;
        const visits_array = ctx_data.dataset.visits.split(",");

        const ccolors = [
            "#00508B",
            "#005C95",
            "#2669A0",
            "#4579AB",
            "#6089B7",
            "#7D99C1",
            "#96ADCF",
            "#B1C0DD"
        ];

        const data = {
            labels: labels_array,
            datasets: [{
                label: lvisits_value,
                data: visits_array,
                fill: false,
                backgroundColor: '#16a0fc',
                tension: 0.1
            }, {
                label: lvisitors_value,
                data: visitors_array,
                fill: false,
                backgroundColor: '#7b4fbc',
                tension: 0.1
            }]
        };

        const config = {
            type: 'bar',
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

chart_referrals();
