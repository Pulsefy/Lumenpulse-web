import { Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { TypeOrmModule } from '@nestjs/typeorm';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { TestExceptionController } from './test-exception.controller';
import { StellarController } from './stellar/stellar.controller';
import { StellarService } from './stellar/stellar.service';
import stellarConfig from './stellar/config/stellar.config';

@Module({
  controllers: [AppController, TestExceptionController, StellarController],
  providers: [AppService, StellarService],
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      envFilePath: '.env',
      load: [stellarConfig],
    }),
    TypeOrmModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: (configService: ConfigService) => ({
        type: 'postgres',
        host: configService.get<string>('DB_HOST'),
        port: configService.get<number>('DB_PORT'),
        username: configService.get<string>('DB_USERNAME'),
        password: configService.get<string>('DB_PASSWORD'),
        database: configService.get<string>('DB_DATABASE'),
        entities: [__dirname + '/**/*.entity{.ts,.js}'],
        synchronize: false, // Always false for production, and recommended false when using migrations
        migrations: [__dirname + '/migrations/*{.ts,.js}'],
        logging: true,
      }),
      inject: [ConfigService],
    }),
  ],
})
export class AppModule {}
