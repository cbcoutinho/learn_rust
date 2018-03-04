import numpy as np

def create_csv():
    filename = 'test.csv'

    N = 5
    fruits = [
        'apple',
        'orange',
        'grape',
        'watermelon',
    ]

    fruit_num = {}
    for fruit in fruits:
        fruit_num[fruit] = 0

    with open(filename, mode='w') as file:

        file.write(str(N) + '\n')

        for _ in range(N):
            rand = np.random.randint(0, len(fruits))
            fruit = fruits[rand]
            num = np.random.randint(10)

            fruit_num[fruit] += num

            # Write the string
            file.write(
                fruits[rand] + ', ' + str(num) + '\n',
            )

        file.write('\n')

        for fruit in fruits:
            file.write(fruit + ', ' + str(fruit_num[fruit]) + '\n')

if __name__ == '__main__':
    create_csv()
