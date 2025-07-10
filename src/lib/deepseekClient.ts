export class DeepSeekClient {
  constructor(private apiKey: string, private endpoint = "https://api.deepseek.com/v1") {}

  async generate(prompt: string) {
    const res = await fetch(`${this.endpoint}/generate`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${this.apiKey}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ model: "DeepSeek-R1", prompt }),
    });
    if (!res.ok) {
      throw new Error(`DeepSeek error ${res.status}`);
    }
    return res.json();
  }
}
