const index = require('./index');

describe('Math module', function () {
    describe('add function', function () {
        // it('should add two numbers correctly', function () {
        //     const result = add(2, 3);
        //     console.log(result)
        // });
        //
        // it('should handle negative numbers', function () {
        //     const result = add(-1, 1);
        //     console.log(result)
        // });

        it('should handle decimal numbers', function () {
            const result = index.temp_add(0.1, 0.2);
            console.log(result)
        });
    });
});

function add(a, b) {
    return a + b;
}
