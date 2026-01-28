import { SetMetadata } from '@nestjs/common';
import { createParamDecorator, ExecutionContext } from '@nestjs/common';
import { User } from 'src/entities/user.entity';

/**
 * Decorator to mark routes as public (skip JWT authentication)
 */
export const Public = () => SetMetadata('isPublic', true);

/**
 * Decorator to get the current authenticated user
 * Usage: @GetUser() user: User
 */
export const GetUser = createParamDecorator(
  (data: unknown, ctx: ExecutionContext): User => {
    const request = ctx.switchToHttp().getRequest();
    return request.user;
  },
);

/**
 * Decorator to get the user's Stellar public key
 * Usage: @GetStellarPublicKey() publicKey: string
 */
export const GetStellarPublicKey = createParamDecorator(
  (data: unknown, ctx: ExecutionContext): string => {
    const request = ctx.switchToHttp().getRequest();
    return request.user?.stellarPublicKey;
  },
);