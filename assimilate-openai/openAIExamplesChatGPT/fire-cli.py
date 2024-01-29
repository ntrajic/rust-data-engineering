#python fire-cli.py add --x=3 --y=4
import fire

def add(x: int, y: int):
    print(x + y)

if __name__ == '__main__':
    fire.Fire()
