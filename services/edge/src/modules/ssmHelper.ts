import { SSMClient, GetParameterCommand, GetParameterCommandInput } from "@aws-sdk/client-ssm";

export class SimpleStorageManager{

  constructor(private readonly ssmClient: SSMClient) {
    this.ssmClient = ssmClient;
  }

  // Returns a decrypted parameter from SSM
  public async decrypt(parameterName: string): Promise<string> {

    const key = this.removeSsmPrefix(parameterName);

    const parameterInput: GetParameterCommandInput = {
      Name: key,
      WithDecryption: true,
    }

    const command = new GetParameterCommand(parameterInput);
    const ssmValue = await this.ssmClient.send(command);

    return (ssmValue.Parameter || { Value: "" }).Value || "";
  }

  private removeSsmPrefix(parameterName: string): string {
    // "ssm:key" --> "key"
    return parameterName.substring(4).trim();
  }
}
