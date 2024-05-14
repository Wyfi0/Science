let divisor = 1;
let result = 0;

for (let i = 0; i < 1000000000; i++) {
    
    let sub_result = 4 / divisor;

    if (i % 2 == 0) {
        result += sub_result;
    }
    else {
        result -= sub_result;
    }

    divisor += 2;
}

console.log(result.toFixed(50));

