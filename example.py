import flatqube_pair_py.flatqube_pair_py as flatqube_pair_py

tokend_data = [
    flatqube_pair_py.TokenDataInput(
        decimals=9,
        balance=5530869000000000),
    flatqube_pair_py.TokenDataInput(
        decimals=18,
        balance=5514989303312229845534954)
]

token_index = {
    '0' * 64: 0,
    '1' * 64: 1}

amplification_coefficient = flatqube_pair_py.AmplificationCoefficient(
    value=85,
    precision=1)

fee_params = flatqube_pair_py.FeeParams(
    denominator=1000000,
    pool_numerator=3000,
    beneficiary_numerator=0)

pair = flatqube_pair_py.StablePair(tokend_data, token_index,
                                   amplification_coefficient, fee_params,
                                   5711020512957239363328239)
# amount, from, to
res = pair.expected_exchange(12, '0' * 64, '1' * 64)
print(res)
res = pair.expected_spend_amount(12, '0' * 64, '1' * 64)

print(res)
# token balances, lp
pair.update_balances([1000000000000000000000000, 1000000000000000000000000],
                     123)
