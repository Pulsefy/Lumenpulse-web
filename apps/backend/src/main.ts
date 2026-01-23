import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { getConnectionToken } from '@nestjs/typeorm';
import { Connection } from 'typeorm';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  
  // Wait for TypeORM connection to be established
  try {
    const connection = app.get<Connection>(getConnectionToken());
    if (connection.isConnected) {
      console.log('✅ TypeORM Connection established');
      console.log(`📊 Database: ${connection.options.database}`);
      console.log(`🔌 Host: ${connection.options.host}:${connection.options.port}`);
    }
  } catch (error) {
    console.error('❌ TypeORM Connection failed:', error);
  }
  
  const port = process.env.PORT ?? 3000;
  await app.listen(port);
  console.log(`🚀 Application is running on: http://localhost:${port}`);
}
bootstrap();
