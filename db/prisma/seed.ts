import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

async function main() {
  await prisma.wordEntry.createMany({
    data: [
      { priority: 2, word: 'Hello', meaning: 'こんにちは', learning_history: '{}' },
      { priority: 1, word: 'World', meaning: '世界', learning_history: '{}' },
      { priority: 3, word: 'Rust', meaning: '錆', learning_history: '{}' },
      { priority: 4, word: 'Programming', meaning: 'プログラミング', learning_history: '{}' },
      { priority: 20, word: 'prerequisite', meaning: '前提条件', learning_history: '{}' },
      { priority: 30, word: 'subsequently', meaning: 'その後', learning_history: '{}' },
      { priority: 40, word: 'consequently', meaning: 'その結果', learning_history: '{}' },
      { priority: 50, word: 'therefore', meaning: 'したがって', learning_history: '{}' },
    ],
  });
}

main()
    .then(async () => {
        console.log('Seed data created successfully');
        await prisma.$disconnect();
        console.log('Disconnected');
    })
    .catch(async (e) => {
        console.error(e);
        await prisma.$disconnect();
        process.exit(1)
    })

