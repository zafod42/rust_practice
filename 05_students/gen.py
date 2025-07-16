import random

names = ['Alex', 'Bill', 'Bobby', 'Robert', 'Paul', 'Stephany', 'Dmitri', 'Nicolas']
exams = ['Physics', 'Math', 'Science', 'History']

for i, name in enumerate(names):
    print(f"Student {name}")
    for j in range(0, random.randint(0, 6)):
        grade = round(random.random() * 5.0, 2)
        exam = random.choice(exams)

        print(f"{exam} {grade}")
    for j in range(0, random.randint(0,2)):
        print()


