import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { Logger } from '@nestjs/common';
import { DataSource } from 'typeorm';
import { getDataSourceToken } from '@nestjs/typeorm';

async function bootstrap() {
  const logger = new Logger('Bootstrap');
  
  try {
    const app = await NestFactory.create(AppModule);

    // Verify database connection (TypeORM initializes automatically with NestJS)
    try {
      const dataSource = app.get<DataSource>(getDataSourceToken());
      if (dataSource.isInitialized) {
        logger.log('✅ TypeORM Connection established successfully');
        logger.log(`📊 Connected to database: ${dataSource.options.database} at ${dataSource.options.host}:${dataSource.options.port}`);
      }
    } catch (error) {
      logger.error('❌ Failed to establish TypeORM connection', error);
      throw error;
    }

    const port = process.env.PORT ?? 3000;
    await app.listen(port);
    logger.log(`🚀 Application is running on: http://localhost:${port}`);
  } catch (error) {
    logger.error('❌ Failed to start application', error);
    process.exit(1);
  }
}
bootstrap();
