import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { User } from './user.entity';
import { UpdateUserDto } from './dtos/update-user.dto';

@Injectable()
export class UsersService {
  constructor(
    @InjectRepository(User)
    private usersRepository: Repository<User>,
  ) {}

  async findById(userId: string): Promise<User | null> {
    return this.usersRepository.findOne({ where: { id: userId } });
  }

  async update(
    userId: string,
    updateUserDto: UpdateUserDto,
  ): Promise<User | null> {
    const user = await this.usersRepository.findOne({ where: { id: userId } });

    if (!user) {
      return null;
    }

    // Prevent password update
    const { password, ...allowedUpdates } = updateUserDto as any;

    Object.assign(user, allowedUpdates);
    return this.usersRepository.save(user);
  }
}
