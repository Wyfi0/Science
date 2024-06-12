-- Hiii owo
Divisor = 1
Result = 0

for i = 0, 1000000000 , 1 do

    Sub_result = 4 / Divisor;

    if i % 2 == 0 then
        Result = Result + Sub_result
    else
        Result = Result - Sub_result
    end

    Divisor = Divisor + 2
end

print(Result)
