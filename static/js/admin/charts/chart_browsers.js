
function chart_browsers() {
    const ctx = getId('chart-browsers');

    if (ctx !== null) {
        const ctx_data = getId('chart-browsers-data');
        const title_value = ctx_data.dataset.title;
        const labels_array = ctx_data.dataset.labels.split(",");
        const lvisitors_value = ctx_data.dataset.lvisitors;
        const visitors_array = ctx_data.dataset.visitors.split(",");

        var ccolors = [];
        for (let i = 0; i < labels_array.length; i++) {
            switch (labels_array[i]) {
                case "Firefox": ccolors.push("#8e5ea2"); break;
                case "Chrome": ccolors.push("#66c948"); break;
                case "Brave": ccolors.push("#f7b22a"); break;
                case "Opera": ccolors.push("#fc5353"); break;
                case "Safari": ccolors.push("#aaaaaa"); break;
                case "Edge": ccolors.push("#3e95cd"); break;
                case "Midori": ccolors.push("#adffb1"); break;
                case "IceCat": ccolors.push("#adc5f3"); break;
                default: ccolors.push("#000000");  // (?)
            }
        }

        const data = {
            labels: labels_array,
            datasets: [{
                label: lvisitors_value,
                data: visitors_array,
                backgroundColor: ccolors
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

chart_browsers();
