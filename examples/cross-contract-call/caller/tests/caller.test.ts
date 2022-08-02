import * as fs from 'fs';
import * as path from "path";
import { ContractRunnerProvider, ContractArgumentFormat } from 'idena-contract-runner-sdk';

it('can deploy and call invoke', async () => {
    const incFuncContractPath = path.join('..', 'function', 'build', 'debug.wasm')
    const sumFuncContractPath = path.join('build', 'debug.wasm')
    const provider = ContractRunnerProvider.create('http://localhost:3333', '');
    await provider.Chain.generateBlocks(1)

    const incFuncContract = fs.readFileSync(incFuncContractPath);
    const incFuncContractEstimateDeployReceipt = await provider.Contract.estimateDeploy("99999", "9999", incFuncContract)
    const incFuncContractDeployTx = await provider.Contract.deploy("99999", "9999", incFuncContract)
    await provider.Chain.generateBlocks(1)

    const incFuncContractDeployReceipt = await provider.Chain.receipt(incFuncContractDeployTx)
    expect(incFuncContractDeployReceipt.success).toBe(true)
    expect(incFuncContractDeployReceipt.contract).toBe(incFuncContractEstimateDeployReceipt.contract)
    const incFuncContractAddress = incFuncContractDeployReceipt.contract

    const sumFuncContract = fs.readFileSync(sumFuncContractPath);

    const sumFuncContractDeployTx = await provider.Contract.deploy("99999", "9999", sumFuncContract, [{
        index: 0,
        format: ContractArgumentFormat.Hex,
        value: incFuncContractAddress,
    }])
    await provider.Chain.generateBlocks(1)

    const sumFuncContractDeployReceipt = await provider.Chain.receipt(sumFuncContractDeployTx)
    expect(sumFuncContractDeployReceipt.success).toBe(true)
    const sumFuncContractAddress = sumFuncContractDeployReceipt.contract

    const sumFuncContractCallInvokeTx = await provider.Contract.call(sumFuncContractAddress, "invoke", "0", "9999", [{
        index: 0,
        format: ContractArgumentFormat.Uint64,
        value: "1",
    },  {
        index: 1,
        format: ContractArgumentFormat.Uint64,
        value: "2",
    }])

    console.log(`sumFuncContractCallInvokeTx: ${sumFuncContractCallInvokeTx}`)
    await provider.Chain.generateBlocks(1)
    const sumFuncContractCallInvokeReceipt = await provider.Chain.receipt(sumFuncContractCallInvokeTx)
    expect(sumFuncContractCallInvokeReceipt.success).toBe(true)

});