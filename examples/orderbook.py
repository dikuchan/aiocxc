import asyncio
import os

import aiocxc

KEY = os.getenv("BINANCE_KEY")
SECRET = os.getenv("BINANCE_SECRET")


async def main():
    connector = aiocxc.binance(KEY, SECRET)
    orderbook = await connector.orderbook("ETHBTC", 1000)
    print(orderbook.nonce)


if __name__ == "__main__":
    asyncio.run(main())
